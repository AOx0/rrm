mod path_finder;
mod xml_analyzer;

#[macro_export]
macro_rules! expand {
    ($path: expr) => {
        {
            #[cfg(feature = "tt")]
            use path_absolutize::Absolutize;

            #[cfg(not(feature = "tt"))]
            use rwm_list::Absolutize;

            $path.absolutize().unwrap_or_else(|error|{
                eprintln!("Failed to expand path with error:\n{}", error);
                std::process::exit(1);
            })
        }
    };
}

pub use path_absolutize::Absolutize;

