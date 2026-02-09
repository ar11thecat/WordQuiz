use std::env;
use std::path::PathBuf;
use image::DynamicImage;

pub struct Resources {
    FAIL_REACTION: DynamicImage,
    CHEER_REACTION: DynamicImage,
    MID_REACTION: DynamicImage,
    CRINGE_REACTION: DynamicImage,
}

impl Resources {

    pub fn new() -> Self {

        let exe_dir = env::current_exe()
            .expect("Failed to get executable path")
            .parent()
            .expect("Failed to get executable directory path")
            .to_path_buf();

        println!("exe directory path: {}", exe_dir.display());

        let data_dir = exe_dir.join("data");
        
        Resources {
            FAIL_REACTION: image::open(&data_dir.join("laughing_cat.jpeg")).unwrap(),
            CHEER_REACTION: image::open(&data_dir.join("laying_cat.jpeg")).unwrap(),
            MID_REACTION: image::open(&data_dir.join("mid_cat.png")).unwrap(),
            CRINGE_REACTION: image::open(&data_dir.join("screaming_cat.jpeg")).unwrap(),
        }
    }

    pub fn fail(&self) -> &DynamicImage {
        return &self.FAIL_REACTION
    }

    pub fn cheer(&self) -> &DynamicImage {
        return &self.CHEER_REACTION
    }

    pub fn mid(&self) -> &DynamicImage {
        return &self.MID_REACTION
    }
    
    pub fn cringe(&self) -> &DynamicImage {
        return &self.CRINGE_REACTION
    }
    
}
