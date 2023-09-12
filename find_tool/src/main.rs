use regex::Regex;
use std::env;
use std::process;

mod tem;

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() != 3{
        eprintln!("使用方式: {} <目标目录> <搜索的正则表达式>",args[0]);
        process::exit(1);
    }

    let pattern: &String= &args[2];
    let regex = match Regex::new(pattern){
        Ok(re) => re,
        Err(err) => {
            eprintln!("无效正则表达式 '{}':{}",pattern,err);
            process::exit(1);
        }
    };
    match tem::find(&args[1],&regex){
        Ok(matches)=>{
            if matches.is_empty() {
                println!("未找到匹配项");
            } else {
                println!("找到以下匹配项：");
                for file in matches{
                    println!("{}",file);
                }
            }

        }
        Err(error) =>{
            eprintln!("发生错误：{}",error);
            process::exit(1);
        }
    }
}


