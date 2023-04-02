struct HelloReq {
    1: required i64 id,
    2: required string name (pilota.rust_type = "string"),
    3: optional set<string> tags,
}

struct UserInfo {
    1: required string name,
    2: required string location,
}

struct HelloItem {
    1: required i64 id,
    2: required string intro,
    3: required UserInfo user_info,
}

struct HelloResp {
    1: required list<HelloItem> data,
}