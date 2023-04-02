pub mod model {
    #![allow(warnings, clippy::all)]
    pub mod model {
        #[derive(
            Debug,
            Default,
            :: pilota :: serde :: Serialize,
            :: pilota :: serde :: Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct HelloReq {
            pub id: i64,
            pub name: ::std::string::String,
            pub tags: ::std::option::Option<::std::collections::HashSet<::pilota::FastStr>>,
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            :: pilota :: serde :: Serialize,
            :: pilota :: serde :: Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct UserInfo {
            pub name: ::pilota::FastStr,
            pub location: ::pilota::FastStr,
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            :: pilota :: serde :: Serialize,
            :: pilota :: serde :: Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct HelloItem {
            pub id: i64,
            pub intro: ::pilota::FastStr,
            pub user_info: UserInfo,
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            :: pilota :: serde :: Serialize,
            :: pilota :: serde :: Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct HelloResp {
            pub data: ::std::vec::Vec<HelloItem>,
        }
    }
}
