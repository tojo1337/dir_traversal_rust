pub mod parallel_traverse {
    use std::{collections::VecDeque, fs::read_dir, sync::mpsc::{channel, Sender}};

    use threadpool::ThreadPool;

    use crate::{DiscoveryStat, FoundItem};

    pub fn traverse_mult(entry: String, t_count: usize) -> VecDeque<String>{
        let mut dir_list = VecDeque::new();
        let mut file_list = VecDeque::new();

        let t_pool = ThreadPool::new(t_count);
        
        dir_list.push_back(entry);
        
        while dir_list.len() > 0 {
            let (cx, rx) = channel();

            let mut dir_list_clone = dir_list.clone();
            let mut i = 0;
            let size = dir_list_clone.len();

            while i< size {
                if let Some(entry_item) = dir_list_clone.pop_front(){

                    let _ = dir_list.pop_front();
                    let sender = cx.clone();
                    
                    t_pool.execute(move || {heper_fn(entry_item, sender);});
                }
                    
                i += 1;
            }

            drop(cx);

            while let Ok(data) = rx.recv() {
                match data.status {
                    DiscoveryStat::File =>{
                        file_list.push_back(data.item);
                    },
                    DiscoveryStat::Dir =>{
                        dir_list.push_back(data.item);
                    },
                    DiscoveryStat::None =>{
                        continue;
                    }
                }
            }
        }

        return file_list;
    }

    fn heper_fn(entry: String, sender: Sender<FoundItem>){
        match read_dir(entry) {
            Ok(items)=>{
                for item in items {
                    match item {
                        Ok(data)=>{
                            let path = data.path();
                            if path.is_dir() {
                                if let Some(str_data) = path.to_str() {
                                    let payload = FoundItem{status: DiscoveryStat::Dir, item: String::from(str_data)};
                                    let _ = sender.send(payload).unwrap_or_else(|_err| {()});
                                }
                            }else {
                                if let Some(str_data) = path.to_str() {
                                    let payload = FoundItem{status: DiscoveryStat::File, item: String::from(str_data)};
                                    let _ = sender.send(payload).unwrap_or_else(|_err|{()});
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
        
}
