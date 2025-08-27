use config::{Config, ConfigError};
use serde::Deserialize;
use std::sync::LazyLock;

pub static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| {
    let config = load_config().unwrap();
    eprintln!("加载配置成功：{:#?}", config);
    config
});
pub static PLAYER_HTML: LazyLock<String> = LazyLock::new(|| {
    let html = r#"
    <!DOCTYPE html>
<html lang="zh-CN">
<head>
  <meta charset="UTF-8" />
  <title>视频播放</title>
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/dplayer/dist/DPlayer.min.css" />
  <style>
    html, body {
      margin: 0;
      padding: 0;
      height: 100%;
      background-color: #000;
      overflow: auto; /* 允许滚动 */
    }

    #dplayers {
      width: 100%;
      min-height: 100%; /* 确保容器至少占满视口高度 */
      display: flex;
      flex-direction: column; /* 垂直排列播放器 */
      gap: 20px; /* 播放器之间的间距 */
      padding: 20px; /* 容器内边距 */
      box-sizing: border-box;
    }

    .player-container {
      width: 100%;
      max-width: 800px; /* 限制播放器最大宽度 */
      aspect-ratio: 16 / 9; /* 保持视频比例 */
      margin: 0 auto; /* 居中 */
    }
  </style>
</head>
<body>
  <div id="dplayers"></div>

  <script src="https://cdn.jsdelivr.net/npm/hls.js@latest"></script>
  <script src="https://cdn.jsdelivr.net/npm/dplayer/dist/DPlayer.min.js"></script>
  <script>
    const videoUrl = '#####';
    const videoUrls = videoUrl.split('%&%&');

     if (videoUrls.length === 0) {
      document.body.innerHTML = '<h2 style="color:white;text-align:center">缺少视频地址</h2>';
    } else {
      const playersContainer = document.getElementById("dplayers");

      videoUrls.forEach((videoUrl, index) => {
        // 创建容器
        const div = document.createElement("div");
        div.id = "dplayer-" + index;
        div.className = "player-container";
        playersContainer.appendChild(div);

        // 初始化 DPlayer
        new DPlayer({
          container: div,
          autoplay: false,
          video: {
            url: videoUrl,
            type: "customHls",
            customType: {
              customHls: function (video, player) {
                if (Hls.isSupported()) {
                  const hls = new Hls();
                  hls.loadSource(video.src);
                  hls.attachMedia(video);
                } else if (video.canPlayType("application/vnd.apple.mpegurl")) {
                  video.src = video.src;
                }
              }
            }
          }
        });
      });
    }
  </script>
</body>
</html>
    "#;
    html.to_string()
});
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub config: InnerConfig,
}

#[derive(Debug, Deserialize)]
pub struct InnerConfig {
    pub heiliao: String,
    pub meiridasai: String,
    pub caoliu: String,
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();
    Ok(settings.try_deserialize::<AppConfig>().unwrap())
}
