
Initializing turbo repo similar in rust, use these step 
1 -> create Cargo.toml file and add this in file 
            [Workspace]
            resolver = "3" 
            memebers = ["api","frontend"]
2 -> create end project similiar as apps/api in node 
    cargo new api (http/backend)


4 -> install poem library for create http routes and also install tokio for asynchronous runtime 