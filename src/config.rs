use std::{fs::{self, File}, io::Write};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    sitebasicinfo: SiteBasicInfo,
    mylinks: MyLinks,
}


#[derive(Clone, Deserialize, Serialize)]
struct SiteBasicInfo {
    title: String,
    subtitle: String,
    description: String,
    author: String,
    favicon: String,
    avatar: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct MyLinks {
    github: String,
    bilibili: String,
    zhihu: String,
    qq: String,
    wechat: String,
    gitee: String,
}

impl Default for Config {
    fn default() -> Self {
        let config_path = "./config.json";
        match fs::read_to_string(config_path) {
            Ok(config_content) => {
                serde_json::from_str(&config_content).expect("config.json not ready")
            },
            Err(_) => {
                println!("not found config.json");
                let res = Config {
                    sitebasicinfo: SiteBasicInfo::default(),
                    mylinks: MyLinks::default(),
                    
                };
                res.store();
                res
            }
        }
    }
}

impl Config {
    fn store(&self) {
        let content = serde_json::to_string_pretty(self).unwrap();
        let mut file = File::create("config.json").unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

}

impl Default for SiteBasicInfo {
    fn default() -> Self {
        SiteBasicInfo {
            title: String::default(),
            subtitle: String::default(),
            description: String::default(),
            author: String::default(),
            favicon: String::default(),
            avatar: String::default(),
        }
    }
}

impl Default for MyLinks {
    fn default() -> Self {
        Self {
            github: String::default(),
            bilibili: String::default(),
            zhihu: String::default(),
            qq: String::default(),
            wechat: String::default(),
            gitee: String::default(),
        }
    }
}

impl Config {
    // 更新 Config
    pub fn update(&mut self, new_info: PartialConfig) {
        if let Some(sitebasicinfo) = new_info.sitebasicinfo {
            self.sitebasicinfo.update(sitebasicinfo);
        }
        if let Some(mylinks) = new_info.mylinks {
            self.mylinks.update(mylinks);
        }
        self.store();
    }
}

impl SiteBasicInfo {
    // 更新 SiteBasicInfo
    pub fn update(&mut self, new_info: PartialSiteBasicInfo) {
        if let Some(title) = new_info.title {
            self.title = title;
        }
        if let Some(subtitle) = new_info.subtitle {
            self.subtitle = subtitle;
        }
        if let Some(description) = new_info.description {
            self.description = description;
        }
        if let Some(author) = new_info.author {
            self.author = author;
        }
        if let Some(favicon) = new_info.favicon {
            self.favicon = favicon;
        }
        if let Some(avatar) = new_info.avatar {
            self.avatar = avatar;
        }
    }
}

impl MyLinks {
    // 更新 MyLinks
    pub fn update(&mut self, new_info: PartialMyLinks) {
        if let Some(github) = new_info.github {
            self.github = github;
        }
        if let Some(bilibili) = new_info.bilibili {
            self.bilibili = bilibili;
        }
        if let Some(zhihu) = new_info.zhihu {
            self.zhihu = zhihu;
        }
        if let Some(qq) = new_info.qq {
            self.qq = qq;
        }
        if let Some(wechat) = new_info.wechat {
            self.wechat = wechat;
        }
        if let Some(gitee) = new_info.gitee {
            self.gitee = gitee;
        }
    }
}

// 定义部分更新用的结构体
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PartialConfig {
    pub sitebasicinfo: Option<PartialSiteBasicInfo>,
    pub mylinks: Option<PartialMyLinks>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PartialSiteBasicInfo {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub favicon: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PartialMyLinks {
    pub github: Option<String>,
    pub bilibili: Option<String>,
    pub zhihu: Option<String>,
    pub qq: Option<String>,
    pub wechat: Option<String>,
    pub gitee: Option<String>,
}