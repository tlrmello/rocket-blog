#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::http::ContentType;

#[get("/")]
fn home() -> (ContentType, String) {
    (
        ContentType::HTML,
        format!(
            r#"
<!doctype html>
<html>
<head>
    <title>tlrmello</title>

    <meta charset="utf-8" />
    <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style type="text/css">
    body {{
        background-color: #f0f0f2;
        margin: 0;
        padding: 0;
        font-family: -apple-system, system-ui, BlinkMacSystemFont, "Segoe UI", "Open Sans", "Helvetica Neue", Helvetica, Arial, sans-serif;
        
    }}
    div {{
        width: 600px;
        margin: 5em auto;
        padding: 2em;
        background-color: #fdfdff;
        border-radius: 0.5em;
        box-shadow: 30px 30px teal;
    }}
    a:link, a:visited {{
        color: #38488f;
        text-decoration: none;
    }}
    @media (max-width: 700px) {{
        div {{
            margin: 0 auto;
            width: auto;
        }}
    }}
    .responsive {{
        width: 100%;
        height: auto;
    }}
    </style>    
</head>

<body>
<div>
<p>👋</p><p>Hi, I'm Taylor (<a href="https://twitter.com/tlrmello">tw: @tlrmello</a> | <a href="https://github.com/tlrmello">gh: @tlrmello</a>). Here's my picture.</p>
<img src="/image/IMG_0329.jpg" alt="Taylor" class="responsive">
<p>I built this page using the <a href="https://rocket.rs/">Rocket</a> web framework for Rust!</p>
</div>
</body>
</html>
"#
        ),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home])
        .mount("/image", FileServer::from("../files"))
}
