pub mod helperfunctions{
    use std::path::PathBuf;
    use crate::{
        daisy_conf::{Config}, daisy_dr::{Router}
    };



    ///This function checks if the requested  path
    /// is a CSS content or not
    pub fn css_request_parser( mut _path: &str) -> bool{
        if _path.ends_with(".css"){
            true
        }
        else{
            false
        }

    }


    pub fn generate_base(
        config: Config,
        router: Router, 
        host_name: Option<&str>
    ) -> PathBuf{

        if &router.sites.domain == host_name.unwrap()  
            { PathBuf::from(&router.sites.root)
            .canonicalize()
            .unwrap_or_else(|e| {
                eprintln!("Invalid base path: {}", e);
                std::process::exit(1);
            })}

        else{
            PathBuf::from(&config.server.base)
            .canonicalize()
            .unwrap_or_else(|e| {
                eprintln!("Invalid base path: {}", e);
                std::process::exit(1);
            })}
         
    }

}