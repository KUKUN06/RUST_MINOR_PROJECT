use blake3::Hasher;
use std::io;
use indicatif::
{
    ProgressBar,
    ProgressStyle
};
use std::collections::HashMap;
use std::fs::File;
use walkdir::WalkDir;
use std::io::Read;


fn main(){
    //Ask user: current directory or whole filesystem
    use dialoguer::{Input,Confirm};
    use std::env;

    println!("Where do you want to scan for duplicates?");
    println!("1:Current Directory");
    println!("2:Whole file system\nEnter 1 or 2:");

    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Failed to take input");
    let choice:u8=input.trim().parse().unwrap();

    let scan_dir=match choice{
        1=>env::current_dir().unwrap(),
        2=>"/".into(),
        _=>{println!("Invalid choice. Existing.");
        return;
    }
    };

    println!("YOu chose to scan: {}", scan_dir.display());

    let confirm= Confirm::new().with_prompt("Do you want to start scanning?").interact().unwrap();
    if !confirm{
        println!("Aborted");
        return;
    }
   

    //Collect all file paths


    let mut file_paths= Vec::new();
    for entry in WalkDir::new(&scan_dir).into_iter().filter_map(|e|e.ok()){
        let path= entry.path();
        if path.is_file(){
            file_paths.push(path.to_owned());

        }
    }

    //Set up progress bar
    let pb=ProgressBar::new(file_paths.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar().template("
        {msg}[{bar:40.cyan/blue}]{percent}%({eta})").unwrap().progress_chars("##-"),
    );
    pb.set_message("Scanning files");

    //Hash file and detect duplicates
    let mut hash_map:HashMap<String,Vec<String>>=HashMap::new();
    for path in file_paths{
        pb.inc(1);
        if let Ok(mut file)=File::open(&path){
            let mut hasher=Hasher::new();
            let mut buffer=[0u8;4096];
            
            while let Ok(n)=file.read(&mut buffer){
                if n==0{
                    break;}
                hasher.update(&buffer[..n]);
            }
            let hash=hasher.finalize().to_hex().to_string();
            let path_str=path.to_string_lossy().to_string();

            hash_map.entry(hash).or_insert(Vec::new()).push(path_str);
        }
    }
    pb.finish_with_message("Scan complete!");

    //Print Duplicates
    println!("\n----Duplicate Files ----");
    for (_hash,paths) in hash_map{
        if paths.len()>1{
            println!("Duplicate group:");
            for p in paths {
                println!("{}",p);
            }
            println!("------------------------");
        }
    }
}
