pub fn error(msg: &str) -> String {
    let template = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Guru Meditation</title>

  <style>
    html, body {
      margin: 0;
      width: 100%;
      height: 100%;
      background: #000;
      color: #fff;
      font-family: "Courier New", monospace;
    }

    .guru-screen {
      height: 100%;
      display: flex;
      align-items: center;
      justify-content: center;
      background: #000;
    }

    .guru-box {
      width: min(720px, 80vw);
      padding: 14px 20px;
      background: #aa0000;
      color: #fff;
      border: 2px solid #ff0000;
      box-shadow:
        0 0 0 2px #550000,
        0 0 30px rgba(255, 0, 0, 0.15);
      box-sizing: border-box;
    }

    .guru-title {
      font-size: 22px;
      font-weight: bold;
      letter-spacing: 1px;
      margin-bottom: 12px;
    }

    .guru-message {
      font-size: 16px;
      line-height: 1.5;
    }

    .guru-code {
      margin-top: 12px;
      font-size: 15px;
      font-weight: bold;
    }

    .guru-cursor {
      display: inline-block;
      width: 9px;
      height: 17px;
      margin-left: 5px;
      background: #fff;
      vertical-align: -3px;
      animation: blink 1s steps(1) infinite;
    }

    @keyframes blink {
      50% {
        opacity: 0;
      }
    }
  </style>
</head>

<body>
  <div class="guru-screen">
    <div class="guru-box">
      <div class="guru-title">Software Failure. Guru Meditation #00000004,0000AA01</div>

      <div class="guru-message">
            <<msg>>
      </div>

      <div class="guru-code">
        Guru Meditation<span class="guru-cursor"></span>
      </div>
    </div>
  </div>
</body>
</html>
"#;

    let error_html = &template.replace("<<msg>>", msg);
    error_html.to_string()
}
