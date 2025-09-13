pub mod discovery_status {
    pub enum DiscoveryStat {
        File,
        Dir,
        None
    }

    impl DiscoveryStat {
        pub fn value(self) -> i32 {
            match self {
                Self::Dir => 1,
                Self::File => 2,
                Self::None => 0
            }
        }
    }

    pub struct FoundItem {
        pub status: DiscoveryStat,
        pub item: String
    }
}