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
extern crate ecb;
extern crate des;
extern crate aes;

use std::process::{Command,Stdio};
use std::os::windows::process::CommandExt;
use crate::num_traits::ToPrimitive;
use std::error::Error;
use std::cell::RefCell;
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
use std::fs::create_dir_all;
use std::io::prelude::*;
use std::path::Path;
use stdext::function_name;
use walkdir::WalkDir;
use utf8_slice;    //  utf8_slice::slice("holla中国人नमस्ते", 4, 10);   // urf8 方式的切片
use std::env::set_var;
use log;
use tracing::{info, instrument};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};


use aes::Aes128;
use block_modes::{BlockMode, Ecb};

use hex_literal::hex;

use aes::cipher::{block_padding::Pkcs7, block_padding::NoPadding,  BlockDecryptMut, BlockEncryptMut, KeyInit};

type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;
type Aes128EcbDec = ecb::Decryptor<aes::Aes128>;

type Aes128Ecb = Ecb<Aes128, Pkcs7>;


// use env_logger::{Builder, Target};


// 在 所有的 使用到 静态 可变的 变量的地方 都要  使用 unsafe{}    太麻烦
// static  mut  Input_Shell_Path: String = String::new(); 


// 默认的密码 
 const  Encropty_DefaultKey: &str = "zukgit12"; 
 
 //  既 编译  也 运行   ,   可能会 编译失败 无法运行
 const  RustRule_Build_Run_Bat_Name: &str  = "rrust_rule_apply_A1";
 
  //  只 运行   ,   不会编译   一定运行
 const  RustRule_Run_Bat_Name: &str  = "zrrust_rule_run_A1";
  
 
lazy_static! {
    static ref XXVEC: Vec<u8> = vec![0x18u8, 0x11u8];

    static ref XXMAP: HashMap<u32, String> = {
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
	
	
	static ref Cur_ExecuteFile_Path_String: String = {
		    let mut Input_Exe_Item: String = String::new(); 
			let mut arg_index = 0 ;
           for arg in std::env::args() {
			   
			   if arg_index == 0{
				   Input_Exe_Item = String::from(arg.as_str());
				   break;
			   }
	
		  arg_index = arg_index + 1;
        }
		Input_Exe_Item
    };
	
	
	//  当前 rust工程的根目录 C:\Users\zhuzj5\Desktop\zbin\win_rbin\A1_rust_rule\target\debug\A1_rust_rule.exe
	//  C:\Users\zhuzj5\Desktop\zbin\win_rbin\A1_rust_rule
	static ref Cur_Package_Name_String : String = {
		    let mut Input_Exe_Item: String = String::new(); 
			let mut arg_index = 0 ;
           for arg in std::env::args() {
			   
			   if arg_index == 0{
				   Input_Exe_Item = String::from(arg.as_str());
				   break;
			   }
	
		  arg_index = arg_index + 1;
        }
//  当前 rust工程的根目录 C:\Users\zhuzj5\Desktop\zbin\win_rbin\A1_rust_rule\target\debug\A1_rust_rule.exe
		if Input_Exe_Item.contains("\\target\\"){
			// 对 字符串进行 截取
		   
	    let first_index : usize  = match Input_Exe_Item.find("\\target\\"){    //  正向查找
			Some(index) => index ,    // usize 转为 i32 
			None =>  usize::max_value() , 		
		};
	
		let  subStr :&str = utf8_slice::slice(Input_Exe_Item.as_str(), 0, first_index); 
		let mut Input_Package_Path_Item = String::from(subStr);  // C:\Users\zhuzj5\Desktop\zbin\win_rbin\A1_rust_rule
		
		// 只留下 A1_rust_rule的操作 
		
		let sub_length = Input_Package_Path_Item.len();
		
		let end_index : usize  = match Input_Package_Path_Item.rfind("\\"){      // //  反向查找 
			Some(index) => index ,    // usize 转为 i32 
			None =>  usize::max_value() , 		
		};
		
		
		let  subStr_packagename :&str = utf8_slice::slice(Input_Package_Path_Item.as_str(), end_index + 1, sub_length); 
				
			Input_Exe_Item = String::from(subStr_packagename); 
		}
		
		Input_Exe_Item
    };
	
		static ref Cur_Package_Path_String : String = {
				    let mut Input_Exe_Item: String = String::new(); 
			let mut arg_index = 0 ;
           for arg in std::env::args() {
			   
			   if arg_index == 0{
				   Input_Exe_Item = String::from(arg.as_str());
				   break;
			   }
	
		  arg_index = arg_index + 1;
        }
//  当前 rust工程的根目录 C:\Users\zhuzj5\Desktop\zbin\win_rbin\A1_rust_rule\target\debug\A1_rust_rule.exe
		if Input_Exe_Item.contains("\\target\\"){
			// 对 字符串进行 截取
		   
	    let first_index : usize  = match Input_Exe_Item.find("\\target\\"){
			Some(index) => index ,    // usize 转为 i32 
			None =>  usize::max_value() , 		
		};
	
		let  subStr :&str = utf8_slice::slice(Input_Exe_Item.as_str(), 0, first_index); 
		Input_Exe_Item = String::from(subStr);
	
		}
		
		Input_Exe_Item	
			
		};
		
		
		
	
	static ref Input_RuleIndex_I32: i32 = {
		    let mut Input_RuleIndex_I32_Item: i32 = -1;
			let mut arg_index = 0 ;
           for arg in std::env::args() {
			   
			   if arg_index == 2{
				 let mut  rule_index :i32 =  match arg.as_str().replace("_","").replace("*","").replace("#","").trim().parse(){
					 Ok(num) => num ,
					 Err(_) =>  -1 , 
				 };
				 
				 Input_RuleIndex_I32_Item = rule_index;   
				   break;
			   }
	
		  arg_index = arg_index + 1;
        }
		Input_RuleIndex_I32_Item
    };
	
	
	
	static ref Zbin_Path_String: String = {

		
	 let mut user_profile:String = match env::var("USERPROFILE") {
		Ok(userhome) => userhome,
         Err(_) => match env::var("HOME"){
				  Ok(home) => home,
			      Err(_) => String::from("当前无法读取到 $Home 和 $USERPROFILE 用户主页信息"),
		       }
	 };			   
		let mut Zbin_Path_String_Item: String = user_profile + "/Desktop/zbin/";

	
		Zbin_Path_String_Item
    };
	
	//,exefile_endtype : String                 //  当前系统  可执行文件的后缀
	static ref ZExeFile_EndPointType_String: String = {   // 当前可执行文件的后缀
	let mut exefile_endtype_string  = String::from(".exe");
	let mut os_type_enum  = OS_TYPE::Windows;
	let mut os_name: String = match env::var("OS"){
		Ok(system_name) => system_name ,
		 Err(_) => String::from("macos"), 
	};
	
	os_name.make_ascii_lowercase();  // 返回 空   对 自身 进行 修改 
	 if !os_name.contains("win") {
		 exefile_endtype_string =  String::from("");
		 if os_name.contains("mac") {
			 os_type_enum = OS_TYPE::MacOS;
		 } else {
			os_type_enum = OS_TYPE::Linux;
		 }
	 }
      exefile_endtype_string
    };
	
	
	//,temp_txt_path : String                 //  当前 写入的日志的txt 文件的 目录
		static ref ZTemp_TxtFile_Path_String: String = {
			
		let mut user_profile:String = match env::var("USERPROFILE") {
		Ok(userhome) => userhome,
         Err(_) => match env::var("HOME"){
				  Ok(home) => home,
			       Err(_) => String::from("当前无法读取到 $Home 和 $USERPROFILE 用户主页信息"),
		       }
	      };
			
		  let mut txtfile_path_string: String = user_profile + "/Desktop/zbin/G2_Temp_Text.txt";
		  txtfile_path_string
		};
	
	
	 // C:\Users\zhuzj5\Desktop\zbin\win_rbin\A1_rust_rule\target\debug\A1_rust_rule.exe  // 当前可执行文件的路径
	 
	 	static ref ZRustRule_DebugExeFile_Path_String: String = {
			
		let mut user_profile:String = match env::var("USERPROFILE") {
		Ok(userhome) => userhome,
         Err(_) => match env::var("HOME"){
				  Ok(home) => home,
			       Err(_) => String::from("当前无法读取到 $Home 和 $USERPROFILE 用户主页信息"),
		       }
	      };
			
	//	let mut rust_exefile_path_string: String = user_profile + "/Desktop/zbin/win_rbin/A1_rust_rule/target/debug/"+A1_rust_rule;
		let mut rust_exefile_path_string: String = format!("{}{}{}{}{}" , user_profile , "/Desktop/zbin/win_rbin/",*Cur_Package_Name_String,"/target/debug/",*Cur_Package_Name_String);

		
		let mut os_name: String = env::var("OS").unwrap();
	
	os_name.make_ascii_lowercase();  // 返回 空   对 自身 进行 修改 
	 if os_name.contains("win") {   // windows 下 加入   .exe 文件 
		 rust_exefile_path_string = rust_exefile_path_string + ".exe";
	
	 }
	 
	 rust_exefile_path_string
   };
		
		
	 
	static ref ZDesktop_Path_String: String = {
		
			 let mut user_profile:String = match env::var("USERPROFILE") {
		Ok(userhome) => userhome,
         Err(_) => match env::var("HOME"){
				  Ok(home) => home,
			       Err(_) => String::from("当前无法读取到 $Home 和 $USERPROFILE 用户主页信息"),
		       }
	 };
	 
		    let mut ZDesktop_Path_String_Item: String = user_profile + "/Desktop/";
		   ZDesktop_Path_String_Item
    };
	
	
	static ref ZSystem_OS_Enum: OS_TYPE = {
	let mut os_type_enum  = OS_TYPE::Windows;
	let mut os_name: String = match env::var("OS"){
		Ok(system_name) => system_name ,
		 Err(_) => String::from("macos"), 
	};
	
	os_name.make_ascii_lowercase();  // 返回 空   对 自身 进行 修改 
	 if !os_name.contains("win") {
		 if os_name.contains("mac") {
			 os_type_enum = OS_TYPE::MacOS;
		 } else {
			os_type_enum = OS_TYPE::Linux;
		 }
	 }
	  os_type_enum
    };
	
	
	static ref ZSystem_Batch_Type_String: String = {
	let mut batch_name : &str = ".bat"; 
	// \rustlib\src\rust\library\core\src\str\mod.rs
	 // 尼玛  MacOS  没有 对应的 OS , fuck 
	 
	let mut os_name: String = match env::var("OS"){
		Ok(system_name) => system_name ,
		 Err(_) => String::from("macos"), 
	};  
	
	os_name.make_ascii_lowercase();  // 返回 空   对 自身 进行 修改 
	 if !os_name.contains("window") {
		 batch_name = ".sh";
	 }
	 let batname_string = String::from(batch_name);
	batname_string
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


// 输入的  可能的文件的 列表
pub static ref InputFilePath_StringVec: Vec<String> ={
        let mut mInputFilePath_StringVec: Vec<String> = Vec::new();
		    let args = std::env::args();
			let mut arg_index = 0 ;
			let mut cur_shell_path : String = String::from("");
         for arg in args {
		// C:\Users\zhuzj5\Desktop  type=alloc::string::String
		if arg_index == 0{  // arg0= 当前执行文件的路径的
		    arg_index = arg_index + 1;
			continue;
		}
		
		if arg_index == 1{  // arg1= 当前路径的 shell 的 路径
	
		 cur_shell_path = String::from(&arg); 
	

			arg_index = arg_index + 1;
			continue;
		}
		
		
		let mfile_path_A  = Path::new(&arg);
        let mfile_path_A_exist_flag = mfile_path_A.exists();
	
	
		let mString_path_B: String  = format!("{}/{}",cur_shell_path,arg);
		let mfile_path_B  = Path::new(&mString_path_B);
        let mfile_path_B_exist_flag = mfile_path_B.exists();
		
		if mfile_path_A_exist_flag == true {
			 mInputFilePath_StringVec.push(arg);
		} else if mfile_path_B_exist_flag == true {
			mInputFilePath_StringVec.push(mString_path_B);
		}
		arg_index = arg_index + 1;
        }
		mInputFilePath_StringVec
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

#[derive(Debug,Copy, Clone)]
pub enum OS_TYPE {
    Windows = 1,
    Linux = 2,
    MacOS = 3,
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




 

pub trait Rust_BaseRule_Trit {

    fn get_rule_index(&self) -> i32;
	
    fn simple_desc(&self) -> String; // 使用的简单描述 中文的该 rule的使用情况 默认会在 ruleTip 被调用

    fn init_with_input_list_params(&mut self,paramList:Vec<String>) -> bool;  // 初始化输入参数
	
    fn apply_rule_operation(&self
	,apply_rule_index : i32                // 选中的规则
	,is_search_alldir_flag : bool          //  是否全选的标识
	,user_shell_path_string : String       //  当前 程序执行的 shell 路径
	,user_desktop_path_string : String       //  当前系统 的 桌面的路径
	,rust_debug_exe_path_string : String            //  当前 编译出来的 可执行文件  路径
	,user_temptxt_path_string : String       //  当前 存放 Log 的文件的路径
	,exefile_endtype : String                 //  当前系统  可执行文件的后缀    .exe  空
	,batchfile_endtype : String                 //  当前系统  批处理文件的后缀  .sh   .bat 
	,cur_os_type : OS_TYPE                 //  当前 系统类型 
	,shell_inputparam_list:Vec<String> 
	,shell_inputfile_list:Vec<String> 
	,onedir_real_file_list:Vec<String>
	,onedir_dir_file_list:Vec<String> 
	,onedir_type_map:HashMap<String, Vec<String>>	
	,all_real_file_list:Vec<String>
	,all_dir_file_list:Vec<String> 
	,real_file_type_map:HashMap<String, Vec<String>> )   -> bool   ;   // 实际的规则应用 
 	
}


//  特征  有 多 继承  
// trait Rust_RealRule_Trit : Rust_BaseRule_Trit{     // 打开注释 报错   the trait `Rust_BaseRule_Trit` is not implemented for `Add_Environment_To_System_Rule_1`
trait Rust_RealRule_Trit  {   //   一样的 方法  中间 有个缓冲 
	
	fn get_struct_name(&self) -> String;
		
	fn get_rule_index(&self) -> i32;
	
	fn is_all_search(&self) -> bool;
		
		
    fn simple_desc(&self) -> String; // 使用的简单描述 中文的该 rule的使用情况 默认会在 ruleTip 被调用

    fn init_with_input_list_params(&self,paramList:Vec<String>) -> bool;  // 初始化输入参数
	
    fn apply_rule_operation(&self
	,apply_rule_index : i32                // 选中的规则
	,is_search_alldir_flag : bool          //  是否全选的标识
	,user_shell_path_string : String       //  当前 程序执行的 shell 路径
	,user_desktop_path_string : String       //  当前系统 的 桌面的路径
	,rust_debug_exe_path_string : String            //  当前 编译出来的 可执行文件  路径
	,user_temptxt_path_string : String       //  当前 存放 Log 的文件的路径
	,exefile_endtype : String                 //  当前系统  可执行文件的后缀    .exe  空
	,batchfile_endtype : String                 //  当前系统  批处理文件的后缀  .sh   .bat 
	,cur_os_type : OS_TYPE                 //  当前 系统类型 
	,shell_inputparam_list:Vec<String> 
	,shell_inputfile_list:Vec<String> 
	,onedir_real_file_list:Vec<String>
	,onedir_dir_file_list:Vec<String> 
	,onedir_type_map:HashMap<String, Vec<String>>	
	,all_real_file_list:Vec<String>
	,all_dir_file_list:Vec<String> 
	,real_file_type_map:HashMap<String, Vec<String>> )   -> bool   ;   // 实际的规则应用 

}


//═════════════════════════════════════════ 模板 模板 Rule_2_Begin Rule2_Begin  Rule2Begin 模板 模板 ═══════════════════════════════════════════════════════
#[derive(Debug)]
pub struct Test_Rule_2  {
	//_______Common_Var Begin_______  默认 需要实际给到的数据类型
	pub   rule_index:i32  ,
	pub   isneed_all_search: bool ,
    //-----------
	// pub  rule_desc: String ,     通过方法来实际得到
	//_______Common_Var End_______

	pub   test_i32: i32 ,
	pub   test_bool: bool ,
	
    // 各个规则实际可能 需要的  实际的 在运行规则时需要的数据
	//════════ Rule_Var Begin════════
	pub user_input_pathvar_refvec: RefCell<Vec<String>> ,  //   用户输入的 环境变量的值  PATH_D:\ZWin_Software\zbin
	//════════Rule_Var End ════════

 }


impl Test_Rule_2{   // 为 规则 Rule_1 提供 commn_function 
	
	fn new(index:i32 , isallSearch:bool) -> Test_Rule_2 {
		Test_Rule_2{
		    rule_index: index,
		    isneed_all_search: isallSearch ,
		    user_input_pathvar_refvec : RefCell::new(Vec::new()) ,
			test_i32:32,
			test_bool: true  ,
		}

	}

}

impl Rust_RealRule_Trit for Test_Rule_2 {  // 为 规则 Rule_1 提供 trait_function 
	
   fn get_rule_index(&self) -> i32{
	  self.rule_index
	}
	
	fn is_all_search(&self) -> bool{
	  self.isneed_all_search
	}

	fn get_struct_name(&self) -> String {
        format!("{}", get_var_type(&self))
    }
	
	
    fn simple_desc(&self) -> String {
	 let  pre_tag: String = format!("{}{}{}{}  ", RustRule_Run_Bat_Name , *ZSystem_Batch_Type_String , " #_",self.rule_index );
	
	let	simple_desc_1 : String =  format!("{}{}", pre_tag, " PATH_D:\\CTS      ## XXXX_Desc_XXXX" );
	
	let    desc : String = format!("{}\n",simple_desc_1);
	desc
    }

// PATH_D:\ZWin_Software\zbin   
  fn init_with_input_list_params(&self,paramList:Vec<String>)   -> bool {
	  true
  }
  
  
    fn apply_rule_operation(&self
	,apply_rule_index : i32                // 选中的规则
	,is_search_alldir_flag : bool          //  是否全选的标识
	,user_shell_path_string : String       //  当前 程序执行的 shell 路径
	,user_desktop_path_string : String       //  当前系统 的 桌面的路径
	,rust_debug_exe_path_string : String            //  当前 编译出来的 可执行文件  路径
	,user_temptxt_path_string : String       //  当前 存放 Log 的文件的路径
	,exefile_endtype : String                 //  当前系统  可执行文件的后缀    .exe  空
	,batchfile_endtype : String                 //  当前系统  批处理文件的后缀  .sh   .bat 
	,cur_os_type : OS_TYPE                 //  当前 系统类型 
	,shell_inputparam_list:Vec<String> 
	,shell_inputfile_list:Vec<String> 
	,onedir_real_file_list:Vec<String>
	,onedir_dir_file_list:Vec<String> 
	,onedir_type_map:HashMap<String, Vec<String>>	
	,all_real_file_list:Vec<String>
	,all_dir_file_list:Vec<String> 
	,real_file_type_map:HashMap<String, Vec<String>> )  -> bool   {
	println!("════════════ {} begin ════════════ ", function_name!());
		 false
	}
	
  
}
//═════════════════════════════════════════ 模板 模板 RuleEnd RuleEnd  RuleEnd 模板 模板 ═══════════════════════════════════════════════════════



//═════════════════════════════════════════ Rule_1_Begin Rule1_Begin  Rule1Begin ═══════════════════════════════════════════════════════
//  使用 成员组合的 方式来 实现  继承

// 定义子类

#[derive(Debug)]
pub struct Add_Environment_To_System_Rule_1  {
	//_______Common_Var Begin_______  默认 需要实际给到的数据类型
	pub   rule_index:i32  ,
	pub   isneed_all_search: bool ,
    //-----------
	// pub  rule_desc: String ,     通过方法来实际得到
	//_______Common_Var End_______

    // 各个规则实际可能 需要的  实际的 在运行规则时需要的数据
	//════════ Rule_Var Begin════════
	pub user_input_pathvar_refvec: RefCell<Vec<String>> ,  //   用户输入的 环境变量的值  PATH_D:\ZWin_Software\zbin
	//════════Rule_Var End ════════

 }


impl Add_Environment_To_System_Rule_1{   // 为 规则 Rule_1 提供 commn_function 
	
	fn new(index:i32 , isallSearch:bool) -> Add_Environment_To_System_Rule_1 {
		Add_Environment_To_System_Rule_1{
		    rule_index: index,
		    isneed_all_search: isallSearch ,
		    user_input_pathvar_refvec : RefCell::new(Vec::new()) ,
			
		}

	}

}


// <Dog as Animal>::baby_name());    子类调用父类的方法
// <Type as Trait>::function(receiver_if_method, next_arg, ...);
impl Rust_RealRule_Trit for Add_Environment_To_System_Rule_1 {  // 为 规则 Rule_1 提供 trait_function 
	
   fn get_rule_index(&self) -> i32{
	  self.rule_index
	}
	
	fn is_all_search(&self) -> bool{
	  self.isneed_all_search
	}

	fn get_struct_name(&self) -> String {
        format!("{}", get_var_type(&self))
    }
	
	
    fn simple_desc(&self) -> String {
      //   format!("当前规则{} 全搜标识{} 时间={}", self.rule_index, self.isneed_all_search,getYYYYMMdd())
	  let  pre_tag: String = format!("{}{}{}{}", RustRule_Run_Bat_Name , *ZSystem_Batch_Type_String , " #_",self.rule_index );
	
	let	simple_desc_1 : String =  format!("{}{}", pre_tag, "   PATH_D:\\CTS      ## 目录环境地址   对当前给定的路径 D:\\CTS 加到环境变量PATH中" );
	let	simple_desc_2 : String =  format!("{}{}", pre_tag, "   path_D:\\APK      ## 目录环境地址   对当前给定的路径 D:\\APK 加到环境变量PATH中" );

	let    desc : String = format!("{}\n{}\n",simple_desc_1,simple_desc_2);
		desc
    }

// PATH_D:\ZWin_Software\zbin   
  fn init_with_input_list_params(&self,paramList:Vec<String>)   -> bool {
	  
	  let mut avaliable_path_index = 0 ;
	  
	    let mut avaliable_path_vec: Vec<String> =  Vec::new();
		
	  for (param_index, param_item) in paramList.iter().enumerate(){
		 println!("Rule[{}]__Param[{}] == {}" , self.rule_index , param_index  , param_item );
		 
		 if param_item.starts_with("PATH_") || param_item.starts_with("path_") {
			 let mut mod_param_item : String = String::from(param_item.as_str());
			 let mut path_string_item :String = String::from(mod_param_item.as_str().replace("PATH_","").replace("path_","").replace("*","").replace("#","").trim());
			
			let param_dir_path  = Path::new(&path_string_item);

            let param_dir_path_exist = param_dir_path.exists();   // 当前 路径 文件 存在 
			
			let param_dir_path_isdir = param_dir_path.is_dir();
			
			if param_dir_path_exist && param_dir_path_isdir {
			 println!("Rule[{}]__Param[{}]_AvalibleParam[{}] == {}  Path[{}]=={}" , self.rule_index , param_index , avaliable_path_index , param_item  ,avaliable_path_index ,path_string_item);

			// *self.user_input_pathvar_refvec.push(path_string_item);
			
			//(&(self.get_user_avaliable_stringvec_ref())).push(path_string_item);
			//	avaliable_path_vec.push(path_string_item);
				self.user_input_pathvar_refvec.borrow_mut().push(path_string_item);
			 avaliable_path_index = avaliable_path_index + 1;
			}
		 }
		 
	
		
	  }
	 
	 let  avaliable_params_vec : Vec<String>  = self.user_input_pathvar_refvec.borrow().to_vec();
	 let  avaliable_params_count = avaliable_params_vec.len();
	 
	 if avaliable_params_count == 0{
		 println!();
		println!("用户输入的有效参数个数:{} 为0 请检查  Rule【{}】 的输入参数!" , avaliable_params_vec.len(),self.rule_index);
		 return false
	 } else {
		 	 println!();
		 	println!("用户输入的有效参数个数:{}  将执行 Rule【{}】 ApplyRule方法   " , avaliable_params_vec.len() ,  self.rule_index);
		 
		 	  for (pass_index, pass_item) in avaliable_params_vec.iter().enumerate(){
				  
				 println!("Avaliable_Param[{}] == {} " , pass_index , pass_item);
  
			  }
		 
	 }
	 
	   true
  }
  
  
    fn apply_rule_operation(&self
	,apply_rule_index : i32                // 选中的规则
	,is_search_alldir_flag : bool          //  是否全选的标识
	,user_shell_path_string : String       //  当前 程序执行的 shell 路径
	,user_desktop_path_string : String       //  当前系统 的 桌面的路径
	,rust_debug_exe_path_string : String            //  当前 编译出来的 可执行文件  路径
	,user_temptxt_path_string : String       //  当前 存放 Log 的文件的路径
	,exefile_endtype : String                 //  当前系统  可执行文件的后缀    .exe  空
	,batchfile_endtype : String                 //  当前系统  批处理文件的后缀  .sh   .bat 
	,cur_os_type : OS_TYPE                 //  当前 系统类型 
	,shell_inputparam_list:Vec<String> 
	,shell_inputfile_list:Vec<String> 
	,onedir_real_file_list:Vec<String>
	,onedir_dir_file_list:Vec<String> 
	,onedir_type_map:HashMap<String, Vec<String>>	
	,all_real_file_list:Vec<String>
	,all_dir_file_list:Vec<String> 
	,real_file_type_map:HashMap<String, Vec<String>> )  -> bool   {
		println!("════════════ {} begin ════════════ ", function_name!());
		
		
	  let  avaliable_params_vec : Vec<String>  = self.user_input_pathvar_refvec.borrow().to_vec();
			 
		for (pathvar_index, pathvar_item) in avaliable_params_vec.iter().enumerate(){
				  
	 println!("PassVar[{}] == {}  OS={:?}" , pathvar_index , pathvar_item , cur_os_type);
				 
				 if cur_os_type as i32 == OS_TYPE::Windows as i32 {
				 	 println!("Windows 下设置环境变量( 请在 管理员权限 窗口执行该命令 )");
					 // 0x08000000  是 无窗口的 flag 
				 // setx PATH "%PATH%;D:Tools"
				 // setx PATH "D:Tools;%PATH%"
		 let command_string : String = format!("{}{}{}{}{}{}","setx PATH ","\"" , pathvar_item,";" ,  "%PATH%;","\" "); 
		 
		 			 println!("PassVar[{}] == {}  OS={:?}  command={}" , pathvar_index , pathvar_item , cur_os_type,command_string);			 

		let output = Command::new("cmd").creation_flags(0x00000010).arg("/c").arg(command_string.as_str()).stdout(Stdio::piped()).output().expect("cmd exec error!");
	     println!("{}_命令执行结果:\n{}", command_string , String::from_utf8_lossy(&output.stdout)); 

		 } else if cur_os_type as i32  == OS_TYPE::MacOS as i32  {
					 	 println!("MacOS 下设置环境变量  等待实现!");
					 
		 }  else{
				println!("Linux 下设置环境变量  等待实现!"); 
					 
               let output =  Command::new("sh").arg("-c").arg("echo hello").output().expect("failed to execute process");
					 
				 }
 
  
		 }
		
		
		 false
	}
}

//═════════════════════════════════════════ Rule_1_End  Rule1_End  Rule1End ═══════════════════════════════════════════════════════

fn show_allrule_tip( ruleVec :&Vec<&dyn Rust_RealRule_Trit>){
	println!("════════════ {} begin ════════════ ", function_name!());
	println!();
	let rule_count:usize = (*ruleVec).len();
	
		// println!("show_allrule_tip  rule_count={} ", rule_count);
	
		for i in 0..rule_count{
			let  selectedRule :  &dyn Rust_RealRule_Trit  =  (*ruleVec)[i];
			println!("{}", selectedRule.simple_desc());
		}
		
}

//  把 要不要  全搜 目录交给  规则    但 当前桌面的 文件的结合是要  提交的    否则 程序 执行 太慢
fn main() {
		// 只有注册 subscriber 后， 才能在控制台上看到日志输出
    tracing_subscriber::registry().with(fmt::layer()).init();
	show_system_info();
	show_vars_info();
    utf8_slice_test( );
	show_args_info(InputParam_StingVec.to_vec(),InputFilePath_StringVec.to_vec());

let time_stamp_string : String = getYYYYMMdd_HHmmSS();
let new_dir_txt_path_string_1 :String  = format!("{}{}{}{}.txt",r"D:\1A\1\2\1\2\",time_stamp_string,r"\",time_stamp_string);
let new_dir_txt_path_string_2 :String  = format!("{}{}{}{}.mp4",r"D:\1A\1\2\1\2\",time_stamp_string,r"\",time_stamp_string);

    createDecryFile(r"D:\1A\1\2\1\2\1.txt",new_dir_txt_path_string_1.as_str());
	// createDecryFile(r"D:\1A\1\2\1\2\2.mp4",new_dir_txt_path_string_2.as_str());
		
	println!("________________________Rule【{}】 Operation Begin________________________",*Input_RuleIndex_I32);
	
	

	// ══════════════════════ InitRule Begin ══════════════════════
	let mut allRule:Vec<&dyn Rust_RealRule_Trit> = Vec::new();
	
	let mut rule1 = Add_Environment_To_System_Rule_1::new(1,false);
	allRule.push(&rule1);

	// let mut rule2 = Test_Rule_2::new(2,true);   // 模板
	// allRule.push(&rule2);
	// ══════════════════════ InitRule End ══════════════════════
	
	let rule_count:usize = allRule.len();
	if *Input_RuleIndex_I32 < 0 {

		// 打印 当前列表的 tip 

		show_allrule_tip(&allRule);
		println!("当前没有选中具体的 Rule 执行打印各个规则的使用说明!");	
		return ;
	}
	

		
	let  selected_index : usize = match ((*Input_RuleIndex_I32)-1).try_into(){
		Ok(value) => value, 
		Err(_) => {

			show_allrule_tip(&allRule);
			println!("当前规则【{0}】 没找到匹配项 请检测输入的规则序列【{0}】.",((*Input_RuleIndex_I32)-1) );
			return 
		}, 	
	};
	
	if selected_index < 0 || selected_index >= rule_count  {

		show_allrule_tip(&allRule);
		println!("当前没有选中具体的 下表索引={}  规则Index={}  规则总数{}  请检查输入参数!",selected_index ,(*Input_RuleIndex_I32),  rule_count);
			
		return ;
	}
	
	
	 // .get 的方式 总是 报错   
	/* let  selectedRule : &dyn Rust_RealRule_Trit =  match allRule.get(selected_index){  
		Some(rule) => rule , 
		None =>  {
			println!("当前规则【{0}】 没找到匹配项 请检测输入的规则序列【{0}】.",selected_index );
			return 
		}, 
	}; */
	
	let  selectedRule :  &dyn Rust_RealRule_Trit  =  *&allRule[selected_index];   //  尼玛  *& 又到 一起了
	// println!("selected_index={}",selected_index );
	//	println!("selectedRule.simple_desc()={}",selectedRule.simple_desc() );
		println!("选中规则【{0}】 全局搜索标识【{2}】 RuleName={1}",selectedRule.get_rule_index() ,selectedRule.get_struct_name(),selectedRule.is_all_search());

        //  需要 全局搜索
        if selectedRule.is_all_search() {
			
			
			if !selectedRule.init_with_input_list_params(InputParam_StingVec.to_vec()){
			
				show_allrule_tip(&allRule);
		         println!("无法通过规则的 初始化参数方法 init_with_input_list_params(Vec)-> bool 执行失败 \n ═════════Failed【{0}】════════ Run_Rule【{0}】_Failed 请检查输入参数!═════════Failed【{0}】════════  \n选中规则【{0}】 \n全局搜索标识【{2}】 \nRuleName=【{1}】 \nSearchDir=【{3}】",selectedRule.get_rule_index() ,selectedRule.get_struct_name(),selectedRule.is_all_search(),*Input_Shell_Path_String);
				return 
			} 
		
	
			
		 println!();
		 println!("通过规则的 初始化参数方法 init_with_input_list_params(Vec)-> bool 参数检测成功 \n ═════════Pass【{0}】════════ Run_Rule【{0}】_Pass ═════════Pass【{0}】════════  \n选中规则【{0}】 \n全局搜索标识【{2}】 \nRuleName=【{1}】 \nSearchDir=【{3}】",selectedRule.get_rule_index() ,selectedRule.get_struct_name(),selectedRule.is_all_search(),*Input_Shell_Path_String);
		 println!();	
		 
    		println!("开始执行规则【{}】 操作.",*Input_RuleIndex_I32);
    		println!("开始递归搜索当前路径【{}】所有的文件&文件夹.",*Input_Shell_Path_String);
    	   
    	   // 对 指定 路径的 文件夹 返回 这个 文件夹的  1.所有的文件夹集合 Vec   2.所有的文件集合 Vec  3.所有的文件类型组成的Map 数据集合
            let all_file_template:(Vec<String>,Vec<String> ,HashMap::<String,Vec<String>>) = match cal_all_file_template(&*Input_Shell_Path_String){
    		Err(why) => panic!("couldn't get the all file for Path【{}】 why={}", *Input_Shell_Path_String , why),
            Ok(template) => template,
    		};
    		
    		
    		// 获取对 指定 路径的 文件的 Vec 集合  
    		let sub_dirfile_vec:Vec<String>  = match cal_sub_file_template(&*Input_Shell_Path_String){
    		Err(why) => Vec::<String>::new(),
            Ok(dirfile_vec) => dirfile_vec,  
    	   };
    
    	   	println!();	
    	   	println!("sub_dirfile_vec 子目录文件集合类型{} ",get_var_type(&sub_dirfile_vec));
    		println!("sub_dirfile_vec 子目录文件大小{} ",sub_dirfile_vec.len());
    	   
		   	 // 获取当前目录下的文件的类型的Map 
    	    let onlydir_file_template:(Vec<String>,Vec<String> ,HashMap::<String,Vec<String>>) = match cal_onlydir_file_template(&*Input_Shell_Path_String){
    		Err(why) => panic!("couldn't get the all file for Path【{}】 why={}", *Input_Shell_Path_String , why),
            Ok(template) => template,
    		};
    	
    		println!();
    		println!("onedir_file_template  当前路径文件数据元组类型{} ",get_var_type(&onlydir_file_template));
    		println!("当前路径文件夹大小【{}】",onlydir_file_template.0.len());
    		println!("当前路径实体文件大小【{}】 ",onlydir_file_template.1.len());		
    		println!("当前路径实体文件类型数量【{}】 ",onlydir_file_template.2.len());	  // zfilesearch 增加类型的数量的 标识
    		
    		println!();	
    		println!("all_file_template 递归数据元组类型【{}】 ",get_var_type(&all_file_template));
    		println!("递归文件夹大小【{}】 ",all_file_template.0.len());
    		println!("递归实体文件大小【{}】 ",all_file_template.1.len());		
    		println!("递归实体文件类型数量【{}】",all_file_template.2.len());	  // zfilesearch 增加类型的数量的 标识
	
			 println!("═════════开始执行 Rule【{}】的 apply_rule_operation()方法═════════" , selectedRule.get_rule_index());

      let param_rule_index = selectedRule.get_rule_index();
	  let param_is_all_search = selectedRule.is_all_search();
	  let param_input_shell_string : String = format!("{}",*Input_Shell_Path_String);
	  let param_desktop_path_string : String = format!("{}",*ZDesktop_Path_String);
	  let param_temptxt_path_string : String= format!("{}",*ZTemp_TxtFile_Path_String);
	  let param_rustexe_path_string : String= format!("{}",*ZRustRule_DebugExeFile_Path_String);
	  
	  let param_exepoint_type_string : String= format!("{}",*ZExeFile_EndPointType_String);
	  let param_systembatch_type_string : String= format!("{}",*ZSystem_Batch_Type_String);
	
	 selectedRule.apply_rule_operation( param_rule_index , param_is_all_search  ,
	 	param_input_shell_string,param_desktop_path_string,param_temptxt_path_string, param_rustexe_path_string ,
		param_exepoint_type_string,param_systembatch_type_string, *ZSystem_OS_Enum ,
	InputParam_StingVec.to_vec(),InputFilePath_StringVec.to_vec(),
	onlydir_file_template.0,onlydir_file_template.1,onlydir_file_template.2,
	all_file_template.0,all_file_template.1,all_file_template.2);
	
		
				
				
		} else {  //  不需要 全局搜搜
			
			println!("开始执行规则【{}】 不需要全局递归搜索路径【{}】文件 .",*Input_RuleIndex_I32,*Input_Shell_Path_String);
				
				
						   	 // 获取当前目录下的文件的类型的Map 
    	    let onlydir_file_template:(Vec<String>,Vec<String> ,HashMap::<String,Vec<String>>) = match cal_onlydir_file_template(&*Input_Shell_Path_String){
    		Err(why) => panic!("couldn't get the all file for Path【{}】 why={}", *Input_Shell_Path_String , why),
            Ok(template) => template,
    		};
    		
    		println!();
    		println!("onedir_file_template  当前路径文件数据元组类型{} ",get_var_type(&onlydir_file_template));
    		println!("当前路径文件夹大小【{}】",onlydir_file_template.0.len());
    		println!("当前路径实体文件大小【{}】 ",onlydir_file_template.1.len());		
    		println!("当前路径实体文件类型数量【{}】 ",onlydir_file_template.2.len());	  // zfilesearch 增加类型的数量的 标识
    		
			
			if !selectedRule.init_with_input_list_params(InputParam_StingVec.to_vec()){

				show_allrule_tip(&allRule);
		         println!("无法通过规则的 初始化参数方法 init_with_input_list_params(Vec)-> bool 执行失败 \n ═════════Failed【{0}】════════ Run_Rule【{0}】_Failed 请检查输入参数!═════════Failed【{0}】════════  \n选中规则【{0}】 \n全局搜索标识【{2}】 \nRuleName=【{1}】 \nSearchDir=【{3}】",selectedRule.get_rule_index() ,selectedRule.get_struct_name(),selectedRule.is_all_search(),*Input_Shell_Path_String);
				return 
			} 
			
		 println!();
		 println!("通过规则的 初始化参数方法 init_with_input_list_params(Vec)-> bool 参数检测成功 \n ═════════Pass【{0}】════════ Run_Rule【{0}】_Pass ═════════Pass【{0}】════════  \n选中规则【{0}】 \n全局搜索标识【{2}】 \nRuleName=【{1}】 \nSearchDir=【{3}】",selectedRule.get_rule_index() ,selectedRule.get_struct_name(),selectedRule.is_all_search(),*Input_Shell_Path_String);
		 println!();	
		 println!("═════════开始执行 Rule【{}】的 apply_rule_operation()方法═════════" , selectedRule.get_rule_index());
	
	
	let  empty_all_dir_stringvec  :Vec<String>  = Vec::new();
	let  empty_all_realfile_stringvec  :Vec<String>  = Vec::new();
	let  empty_all_type_pathvec_map: HashMap::<String,Vec<String>>  = HashMap::<String,Vec<String>>::new();
	
	
	      let param_rule_index = selectedRule.get_rule_index();
	  let param_is_all_search = selectedRule.is_all_search();
	  let param_input_shell_string : String = format!("{}",*Input_Shell_Path_String);
	  let param_desktop_path_string : String = format!("{}",*ZDesktop_Path_String);
	  let param_temptxt_path_string : String= format!("{}",*ZTemp_TxtFile_Path_String);
	  let param_rustexe_path_string : String= format!("{}",*ZRustRule_DebugExeFile_Path_String);
	  
	  let param_exepoint_type_string : String= format!("{}",*ZExeFile_EndPointType_String);
	  let param_systembatch_type_string : String= format!("{}",*ZSystem_Batch_Type_String);
	
	 selectedRule.apply_rule_operation( param_rule_index , param_is_all_search  ,
	 	param_input_shell_string,param_desktop_path_string,param_temptxt_path_string, param_rustexe_path_string ,
		param_exepoint_type_string,param_systembatch_type_string, *ZSystem_OS_Enum ,
	InputParam_StingVec.to_vec(),InputFilePath_StringVec.to_vec(),
	onlydir_file_template.0,onlydir_file_template.1,onlydir_file_template.2,
	empty_all_dir_stringvec,empty_all_realfile_stringvec,empty_all_type_pathvec_map);
		}
 
}







fn show_args_info(param_vec: Vec<String> , inputfile_vec: Vec<String>  ){
	println!("════════════ {} begin ════════════ ", function_name!());
	let mut param_index = 0 ;
	let param_len = param_vec.len();
	    println!("param_vec_type={}  len=【{}】", get_var_type(&param_vec),param_len);
	    for param in param_vec {
        println!("param[{}][{}]___{}  type={}", param_index ,param_len ,  param,get_var_type(&param));
        param_index = param_index + 1;
    }
	
	println!();
	println!();
	let mut inpufile_index = 0 ;
	let file_len = inputfile_vec.len();
	    println!("InputFilePath_StringVec_Type={}  len=【{}】", get_var_type(&inputfile_vec),file_len);
	    for fileItem in inputfile_vec {
        println!("inpufile[{}][{}]___{}  type={}", inpufile_index ,file_len ,  fileItem,get_var_type(&fileItem));
        inpufile_index = inpufile_index + 1;
    }
	


}



//  //  utf8_slice::slice("holla中国人नमस्ते", 4, 10);   // urf8 方式的切片
fn utf8_slice_test( ){
	
	println!("════════════ {} begin ════════════ ", function_name!());
		
	let str_temp = String::from("holla 一नमस्ते二");
	
	if str_temp.contains("一"){
		
	
	   let mut str_length = str_temp.len();
		let first_index : usize  = match	str_temp.find("一"){
			Some(index) => index ,    // usize 转为 i32 
			None =>  usize::max_value() , 		
		};
	
		let  subStr :&str = utf8_slice::slice(str_temp.as_str(), first_index, str_length);  
		
		
		// 包含一  first_index=6    str_length=30  subStr=一नमस्ते二
	//	println!("包含一  first_index={}    str_length={}  subStr={}",first_index , str_length ,subStr );	

		
			
	} else{
		//	println!("不包含一  ");
	}
	
}


fn show_vars_info( ){
	println!("════════════ {} begin ════════════ ", function_name!());
//  Input_Shell_Path_String.as_str()     同 *Input_Shell_Path_String

	println!("Input_RuleIndex_I32={}   ", *Input_RuleIndex_I32);
	println!();
	println!("Cur_Package_Name_String={}",  *Cur_Package_Name_String);
	println!("Cur_Package_Path_String={}",  *Cur_Package_Path_String);
	println!("Cur_ExecuteFile_Path_String={}",  *Cur_ExecuteFile_Path_String);
	println!();
		
	println!("Input_Shell_Path_String={}",  *Input_Shell_Path_String);
	println!("Zbin_Path_String={} ", *Zbin_Path_String);
	println!("ZDesktop_Path_String={} ", *ZDesktop_Path_String);
	println!("ZRustRule_DebugExeFile_Path_String={} ", *ZRustRule_DebugExeFile_Path_String);
	println!("ZTemp_TxtFile_Path_String={} ", *ZTemp_TxtFile_Path_String);
	println!();
	println!("ZExeFile_EndPointType_String={} ", *ZExeFile_EndPointType_String);
	println!("ZSystem_Batch_Type_String={} ", *ZSystem_Batch_Type_String);
	println!();
	println!("ZSystem_OS_Enum={:?}  ZSystem_OS_Enum_Type={}", *ZSystem_OS_Enum ,get_var_type(&*ZSystem_OS_Enum));


//  println!("getSystem_OS_EnumType()={:?} ", getSystem_OS_EnumType());

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


//  获取当前 目录的 所有子 文件      包括 文件 文件夹
fn cal_sub_file_template( dirFilePath: &str) -> Result<Vec<String>, Box<dyn Error>> {
	let mut sub_dir_pathstring_vec :Vec<String>  = Vec::<String>::new();

    let mut file_index = 0;

    let mut dir_index = 0;

    let mut file_dir_index = 0;

    let path_ReadDir = fs::read_dir(dirFilePath).unwrap();

    for mCurFile in path_ReadDir {
		sub_dir_pathstring_vec.push(mCurFile.as_ref().unwrap().path().display().to_string());
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

	
	Ok(sub_dir_pathstring_vec)

}


fn cal_onlydir_file_template( inputPathStr: &str) -> Result<(Vec<String>,Vec<String>,HashMap::<String,Vec<String>>), Box<dyn Error>> {
		let mut one_dir_dirfile_pathstring_vec :Vec<String>  = Vec::<String>::new();
	let mut one_dir_realfile_pathstring_vec :Vec<String>  = Vec::<String>::new();
	
	let mut onedir_filetype_pathstrvec_map: HashMap::<String,Vec<String>>  = HashMap::<String,Vec<String>>::new();
	
	    let mut file_index = 0;

    let mut dir_index = 0;

    let mut file_dir_index = 0;

    let path_ReadDir = fs::read_dir(inputPathStr).unwrap();

    for mCurFile in path_ReadDir {
	
        if mCurFile.as_ref().unwrap().path().is_file() {
    
			
		let curPath = 	mCurFile.as_ref().unwrap().path();
			
			let mut file_type_str: &str = match curPath.extension(){
               None => "unknow",  //必须处理None, 不能操作，返回None
               Some(mExtension) => match mExtension.to_str(){
				      Some(mExtension_str) => mExtension_str ,
				      None => "unknow", 
			   }, //Some变成加一的Some,仍旧是Option<T>
			};
			
	
			
			let mut file_type_string = String::from(file_type_str);
			file_type_string.make_ascii_lowercase();
	
			let mut curdir_pathvec_value: Vec<String> = match onedir_filetype_pathstrvec_map.get(&file_type_string){
				    Some(mPathVecValue) => mPathVecValue.to_vec() ,
				    None => Vec::<String>::new(), 
			  };
			  
			  curdir_pathvec_value.push(String::from(mCurFile.as_ref().unwrap().path().display().to_string().as_str()));
			  onedir_filetype_pathstrvec_map.insert(file_type_string, curdir_pathvec_value);


			  one_dir_realfile_pathstring_vec.push(mCurFile.as_ref().unwrap().path().display().to_string());

            file_index += 1;
        } else if mCurFile.as_ref().unwrap().path().is_dir() {
			one_dir_dirfile_pathstring_vec.push(mCurFile.as_ref().unwrap().path().display().to_string());
            println!(
                "Dir[{}]={}",
                dir_index,
                mCurFile.unwrap().path().display().to_string()
            );
            dir_index += 1;
        }

        file_dir_index += 1;
    }
	
	Ok((one_dir_dirfile_pathstring_vec,one_dir_realfile_pathstring_vec,onedir_filetype_pathstrvec_map))	
}




fn cal_all_file_template( inputPathStr: &str) -> Result<(Vec<String>,Vec<String>,HashMap::<String,Vec<String>>), Box<dyn Error>> {
	let mut all_dir_pathstring_vec :Vec<String>  = Vec::<String>::new();
	let mut all_realfile_pathstring_vec :Vec<String>  = Vec::<String>::new();
	
	let mut type_pathvec_map: HashMap::<String,Vec<String>>  = HashMap::<String,Vec<String>>::new();
	
	
	let ospath  = Path::new(inputPathStr);
		
	let path_display = ospath.display();
    // 文件是否存在
    let file_exist_flag = ospath.exists();
    let is_file_flag = ospath.is_file();
    let is_dir_flag = ospath.is_dir();
	
	if is_file_flag || !file_exist_flag {
		println!("当前路径【{}】 is_file_flag={} is_exist_flag={} 是实体文件或者不存在无法读取所有数据 ",inputPathStr , is_file_flag , file_exist_flag );
		return  		Ok((all_dir_pathstring_vec,all_realfile_pathstring_vec,type_pathvec_map))
	}
	
	println!("当前路径【{}】 is_dir_flag={}  is_file_flag={} is_exist_flag={} path_display={}  开始执行遍历所有文件的操作 ",inputPathStr ,is_dir_flag, is_file_flag , file_exist_flag  ,path_display ,  );


    let mut file_index = 0;

    let mut dir_index = 0;

    let mut file_dir_index = 0;

// \rustlib\src\rust\library\std\src\path.rs 
    for e in WalkDir::new(inputPathStr).into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {   // e 是结构体 struct `walkdir::DirEntry` 
            all_realfile_pathstring_vec.push(e.path().display().to_string());
			
			let mut file_type_str: &str = match e.path().extension(){
               None => "unknow",  //必须处理None, 不能操作，返回None
               Some(mExtension) => match mExtension.to_str(){
				      Some(mExtension_str) => mExtension_str ,
				      None => "unknow", 
			   }, //Some变成加一的Some,仍旧是Option<T>
			};
			let mut file_type_string = String::from(file_type_str);
			file_type_string.make_ascii_lowercase();
			
            println!("file[{}]={}  filetype={:?}", file_index, e.path().display(), file_type_string);
            file_index += 1;
			
			  let mut cur_pathvec_value: Vec<String> = match type_pathvec_map.get(&file_type_string){
				    Some(mPathVecValue) => mPathVecValue.to_vec() ,
				    None => Vec::<String>::new(), 
			  };
			  
			  cur_pathvec_value.push(String::from(e.path().display().to_string().as_str()));
			  type_pathvec_map.insert(file_type_string, cur_pathvec_value);


        } else if e.metadata().unwrap().is_dir() {
			all_dir_pathstring_vec.push(e.path().display().to_string());
            println!("dir[{}]={}", dir_index, e.path().display());
            dir_index += 1;
        }

        file_dir_index += 1;
    }

    println!( "file[{}] dir[{}] all[{}]", file_index, dir_index, file_dir_index);
	
	Ok((all_dir_pathstring_vec,all_realfile_pathstring_vec,type_pathvec_map))
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
	 println!("");
	    println!("__________  env::vars  begin __________ ");

 let mut var_index = 0 ;
    for (k, v) in env::vars() {
        println!("env::vars[{}]{} _____ {}", var_index , k, v);
		var_index = var_index + 1;
    }
		 println!("");
	 println!("__________  env::args_os  begin __________ ");

	 var_index = 0 ;
   for args_os in   env::args_os() {
        println!("env::args_os[{}] _____ {:?}", var_index , args_os);
		var_index = var_index + 1;
    }
    println!("env::current_dir[{}] _____ {:?}", var_index , env::current_dir());
	var_index = var_index + 1;
	println!("env::home_dir[{}] _____ {:?}", var_index , env::home_dir());
	var_index = var_index + 1;
	println!("env::temp_dir[{}] _____ {:?}", var_index , env::temp_dir());
    var_index = var_index + 1;
	println!("env::current_exe[{}] _____ {:?}", var_index , env::current_exe());
    var_index = var_index + 1;
	
	 println!("");
	println!("__________  env::vars_os()  begin __________ ");
	 var_index = 0 ;
	    for vars_os in   env::vars_os() {
        println!("env::vars_os[{}] _____ {:?}", var_index , vars_os);
		var_index = var_index + 1;
    }
	
    println!("════════════  重要参数 important_system_info begin ════════════ ");

	let mut os_path: String = match env::var("PATH"){
		Ok(system_path) => system_path ,
		Err(_) => String::from("没有 PATH 环境变量"), 
	};
	
    println!("PATH: {}", os_path);
    println!();
	
	let mut os_username: String = match env::var("USERNAME"){
		Ok(system_username) => system_username ,
		Err(_) => String::from("没有 USERNAME 环境变量"), 
	};

    println!("USERNAME: {}", os_username);
    println!();
	let mut os_name: String = match env::var("OS"){
		Ok(system_os) => system_os ,
		Err(_) => String::from("没有 OS 环境变量"), 
	};
    println!("OS: {}", os_name);
    println!();
	
	let mut os_homepath: String = match env::var("HOMEPATH"){
		Ok(system_homepath) => system_homepath ,
		Err(_) => String::from("没有 HOMEPATH 环境变量"), 
	};
	
    println!("HOMEPATH: {}", os_homepath);
    println!();
	
	let mut os_userprofile: String = match env::var("USERPROFILE"){
		Ok(system_userprofile) => system_userprofile ,
		Err(_) => String::from("没有 USERPROFILE 环境变量"), 
	};
	
    println!("USERPROFILE: {}", os_userprofile);
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
    let fmt = "%Y_%m_%d";
    let now = Local::now().format(fmt);
    println!("{}", now);
    return now.to_string();
}

fn getYYYYMMdd_HHmmSS() -> String {
    let fmt = "%Y_%m_%d_%H%M%S";
    let now = Local::now().format(fmt);
    println!("{}", now);
    return now.to_string();
}

fn getTimeLong64() -> i64 {
    return Local::now().timestamp();
}

// \rustlib\src\rust\library\std\src\sys\windows\fs.rs
fn createDecryFile(raw_file_pathstr: &str , target_file_pathstr: &str ) -> bool{   // 解密文件操作
	

     const BYTE_HEAD_LENGTH : usize  = 1024 * 10 * 10; // 读取文件Head字节数常数
	 let head_byte_vec : Vec<u8> = Vec::with_capacity(BYTE_HEAD_LENGTH);


 // pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> 【】
 // pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> 【】
 

	let raw_file_path  = Path::new(&raw_file_pathstr);

	let raw_file_exist = raw_file_path.exists();
		
		if !raw_file_exist {
			println!("当前 原文件【{}】 不存在  无法完成 解密的操作 请检查参数! ",raw_file_pathstr);
			return false 
		}
		
		
		 let mut raw_file = match File::open(raw_file_pathstr){
	       // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
            Err(why) => { println!("不能打开源文件【{}】  原因【{:?}】", raw_file_pathstr, why);  return false } ,
            Ok(file) => file,
		 };
		 
		 
		 
		 let mut raw_file_byte_vec: Vec<u8> = Vec::new();
			
		let raw_bytes_length =  match raw_file.read_to_end(&mut raw_file_byte_vec){
		 Err(why) => { println!("不能从源文件【{}】读取到字节数组  原因【{:?}】", raw_file_pathstr, why);  return false } ,
         Ok(byte_length) => byte_length,	
		};
		
/*	
成功从源文件 【D:\1A\1\2\1\2\1.txt】 读取到【102400】个总字节大小的数据 Vec<u8>!
加密字节大小【102400】  正常字节大小【0】!
当前文件【D:\1A\1\2\1\2\2022_11_02_151414\2022_11_02_151414.txt】 创建成功!
成功从源文件 【D:\1A\1\2\1\2\2.mp4】 读取到【24280344】个总字节大小的数据 Vec<u8>!
加密字节大小【102400】  正常字节大小【24177944】!
当前文件【D:\1A\1\2\1\2\2022_11_02_151414\2022_11_02_151414.mp4】 创建成功!
*/
		
		println!("成功从源文件 【{}】 读取到【{}】个总字节大小的数据 Vec<u8>! ",raw_file_pathstr , raw_bytes_length);
			
		//  从 当前字节读取 BYTE_HEAD_LENGTH 个数据到当前的 集合中
		
	   let mut good_file_byte_head_vec: Vec<u8> = Vec::new();   // 从加密Head 解密得到的数据
			
		let mut raw_file_byte_nohead_vec: Vec<u8> = Vec::new();  //  不在 head 区域的所有的正常的在raw的数据
	
				
		if raw_bytes_length <= BYTE_HEAD_LENGTH{   // 读取到的字节数 小于 HeadLength  那么就把所有的数据 都进行解密
			
			 bad_to_good_byte_operation(&mut raw_file_byte_vec , &mut good_file_byte_head_vec);
			
		} else{
			
		 // raw_file_byte_vec  包含 0..BYTE_HEAD_LENGTH 的 数据  
		 // 剩下的属于   包含在 raw_file_byte_nohead_vec 里 BYTE_HEAD_LENGTH..End
		  raw_file_byte_nohead_vec = 	raw_file_byte_vec.split_off(BYTE_HEAD_LENGTH);
		  bad_to_good_byte_operation(&mut raw_file_byte_vec, &mut good_file_byte_head_vec);
		}
		
		
		println!("加密字节大小【{}】  正常字节大小【{}】! ",raw_file_byte_vec.len(),raw_file_byte_nohead_vec.len());
			
		
				 
			

	let target_file_path  = Path::new(&target_file_pathstr);

		
	let target_file_exist = target_file_path.exists();


if !target_file_exist {
	
    let target_file_parent_path = target_file_path.parent().unwrap();
	
    create_dir_all(target_file_parent_path).unwrap();

    let mut target_file = match File::create(&target_file_path) {   //  会覆盖原有的文件
        Err(why) =>  {println!("当前创建文件【{}】 失败 原因=【{}】",target_file_path.display(),why); return false },
        Ok(file) => file,
    };

	println!("当前文件【{}】 创建成功! ",target_file_path.display());
	
} else {
	println!("当前文件【{}】 已经存在 不用创建! ",target_file_path.display());
}

	true
	
}

	 fn test_enc(){
			    println!("════════════ {} begin ════════════ ", function_name!()); 

// 字符串: 12345678
// 加密密码: zukgit12

// 原始字符串:31 3233343536373831323334353637 38
// 加密字符串:6C F78142080309A86CF7814208 03 09 A8

// 原始字符串_16_【[31, 32, 33, 34, 35, 36, 37, 38, 31, 32, 33, 34, 35, 36, 37, 38]】
// 加密字符串_16_【[b2, de, fe, be, d3, 70, 17, 35, 57, ad, 86, 16, c4, 56, de, c]】
// 解密字符串_16_【[31, 32, 33, 34, 35, 36, 37, 38, 31, 32, 33, 34, 35, 36, 37, 38]】
// let key = [0x7A,0x75,0x6B,0x67,0x69,0x74,0x31,0x32,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00];

// 原始字符串_16_【[31, 32, 33, 34, 35, 36, 37, 38, 31, 32, 33, 34, 35, 36, 37, 38]】
// 加密字符串_16_【[9e, 79, 4b, 8c, f4, fc, 87, d3, a8, 6d, de, 94, bd, 57, 43, 29]】
// 解密字符串_16_【[31, 32, 33, 34, 35, 36, 37, 38, 31, 32, 33, 34, 35, 36, 37, 38]】
// let key = [0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x7A,0x75,0x6B,0x67,0x69,0x74,0x31,0x32];

let key = [0x31,0x32,0x33,0x34,0x35,0x36,0x37,0x38,0x31,0x32,0x33,0x34,0x35,0x36,0x37,0x38];

// 32 ok
// 16 的 倍数 
// let plaintext =      *b"12345678";    // 8_failed
// let plaintext =      *b"1234567812345678";    // 16 ok
// let plaintext =    *b"123456781234567812345678";    // 24_failed
// let plaintext =    *b"12345678123456781234567812345678";    // 32 ok 
// let plaintext =    *b"1234567812345678123456781234567812345678";  // 40 failed   
// let plaintext =    *b"123456781234567812345678123456781234567812345678";  // 48 ok  
// let plaintext =    *b"1234567812345678123456781234567812345678123456781234567812345678";  // 64 ok  
let plaintext =      *b"1234567812345678";    // 16 ok

// println!("原始字符串【{:?}】  \n加密字符串【{:?}】\n解密字符串【{:?}】 ", *b"hello world! this is my plaintext.");

println!("原始字符串_{}_【{:x?}】", plaintext.len(), &plaintext);

// encrypt/decrypt in-place
// buffer must be big enough for padded plaintext






let pt_len = plaintext.len();
const bytesize: usize  = 16;

let mut buf = [0u8; bytesize];

buf[..pt_len].copy_from_slice(&plaintext);
let ct = Aes128EcbEnc::new(&key.into())
    .encrypt_padded_mut::<NoPadding>(&mut buf, pt_len)
    .unwrap();    // 使用 NoPadding  那么必须保证 8的倍数
// assert_eq!(ct, &ciphertext[..]);
println!("加密字符串_{}_【{:x?}】 ",ct.len(), ct);

let pt = Aes128EcbDec::new(&key.into())
    .decrypt_padded_mut::<NoPadding>(&mut buf)
    .unwrap();
// assert_eq!(pt, &plaintext);
println!("解密字符串_{}_【{:x?}】 ",pt.len(), pt);

		
 /*	
// encrypt/decrypt from buffer to buffer
let mut buf = [0u8; 48];
let ct = Aes128EcbEnc::new(&key.into())
    .encrypt_padded_b2b_mut::<Pkcs7>(&plaintext, &mut buf)
    .unwrap();
// assert_eq!(ct, &ciphertext[..]);
println!("b2b_加密字符串_{}_【{:?}】 ",ct.len(), ct);
let mut buf = [0u8; 48];
let pt = Aes128EcbDec::new(&key.into())
    .decrypt_padded_b2b_mut::<Pkcs7>(&ct, &mut buf)
    .unwrap();
// assert_eq!(pt, &plaintext);
println!("b2b_解密字符串_{}_【{:?}】 ",pt.len(), pt);
*/
	    println!("════════════ {} end ════════════ ", function_name!()); 
	 }
	 
	 
// https://asecuritysite.com/symmetric/rust_aes2
//  把 bad 字节转为  good 字节的方法 
fn bad_to_good_byte_operation( bad_byte_vec: &mut Vec<u8>  , good_byte_vec: &mut Vec<u8>)  {
		    println!("════════════ {} begin ════════════ ", function_name!());


	let  DefaultKey: &str = "zukgit12"; 
	 test_enc();
	
	/*
	const Head_Byte_Size: usize  = 1024 * 10 * 10;

    let mut Head_Buff = [0u8; Head_Byte_Size];
    
	// 7A 75 6B 67 69 74 31 32    // zukgit12
	 let Default_Key = "zukgit12";
 let Default_Key = [0x42; 16];
	 let Default_Key = [0x7A,0x75,0x6B,0x67,0x69,0x74,0x31,0x32,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00];
	
    let Default_Iv : [u8; 0] = hex!("");


let pt_len = bad_byte_vec.len();

// let ct = Aes128EcbEnc::new(&Default_Key.into()).encrypt_padded_mut::<NoPadding>( bad_byte_vec, pt_len).unwrap();


let pt = Aes128EcbDec::new(&Default_Key.into()).decrypt_padded_mut::<NoPadding>(bad_byte_vec).unwrap();
	
	
	let result_value =  pt.to_vec() ;
	
	// let text : String = String::from_utf8(result_value);

println!("decrypt_padded_mut_解密字符串_{}_【{:?}】 ",pt.len(), pt);

println!("  result_value={}   " , get_var_type(&result_value) );

println!("   decrypt_padded_mut_解密字符串长度_{}  type&={} " , pt.len(),get_var_type(&pt) );

*/

// NoPadding
//  String strDefaultKey_Rule7 = "zukgit12"
//		Security.addProvider(new com.sun.crypto.provider.SunJCE());
//		Key key = getKey(strDefaultKey_Rule7.getBytes());
//		encryptCipher = Cipher.getInstance("DES/ECB/NoPadding");
//		encryptCipher.init(Cipher.ENCRYPT_MODE, key);
			
			
	// 加密字节数组
// 	public static byte[] encrypt(byte[] arrB) throws Exception {
// 		return encryptCipher.doFinal(arrB);
// 	}
		    println!("════════════ {} end ════════════ ", function_name!());
	
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
