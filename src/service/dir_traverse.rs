pub mod dir_traverse {
    use std::{collections::VecDeque, fs::{read_dir}};

    pub fn traverse(entry: String)-> VecDeque<String>{
        let mut dir_list = VecDeque::new();
        let mut file_list = VecDeque::new();

        dir_list.push_back(entry);

        while dir_list.len() > 0 {
            let mut dir_list_clone = dir_list.clone();
            let mut i = 0;
            let size = dir_list_clone.len();
            while i< size {
                let entry_opt = dir_list_clone.pop_front();
                let _ = dir_list.pop_front();
                
                if let Some(entry) = entry_opt {
                        match read_dir(entry) {
                            Ok(items)=>{
                                for item in items {
                                match item {
                                    Ok(data)=>{
                                        let path = data.path();
                                        if path.is_dir() {
                                            if let Some(str_data) = path.to_str() {
                                                dir_list.push_back(String::from(str_data));
                                            }
                                        }else {
                                            if let Some(str_data) = path.to_str() {
                                                let file_path = String::from(str_data);
                                                file_list.push_back(file_path);
                                            }
                                        }
                                    },
                                    Err(_)=>{}
                                }
                            }
                        },
                        Err(_)=>{}
                    }
                }

                i += 1;
            }
        }
            
        return file_list;
    }
}