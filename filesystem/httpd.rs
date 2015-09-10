use programs::common::*;

pub fn encode(text: &String) -> String{
    let mut html = String::new();

    for c in text.chars() {
        match c {
            '&' => html = html + "&amp;",
            '"' => html = html + "&quot;",
            '<' => html = html + "&lt;",
            '>' => html = html + "&gt;",
            _ => html = html + c
        }
    }

    return html;
}

pub struct Application {
    output: String,
    scroll: Point,
    wrap: bool
}

impl SessionItem for Application {
    fn main(&mut self, url: URL){
        let mut window = Window::new(Point::new((rand() % 400 + 50) as isize, (rand() % 300 + 50) as isize), Size::new(576, 400), "HTTP Server".to_string());

        macro_rules! println {
            ($text:expr) => ({
                self.output.vec.push_all(&$text.vec);
                self.output.vec.push('\n');
                self.draw_content(&mut window);
            });
        }

        println!("Starting HTTP Server".to_string());

        loop {
            let mut resource = URL::from_string(&"tcp:///80".to_string()).open();
            match resource.stat(){
                ResourceType::File => {
                    println!("Request from ".to_string() + resource.url().to_string());

                    let mut data: Vec<u8> = Vec::new();
                    resource.read_to_end(&mut data);
                    println!(String::from_utf8(&data));

                    let html = self.generate_html(&"readme".to_string()).to_utf8();

                    let mut response = ("HTTP/1.1 200 OK\r\n".to_string()
                            + "Content-Type: text/html; charset=UTF-8\r\n"
                            + "Content-Length: " + html.len() + "\r\n"
                            + "Connection: Close\r\n"
                            + "\r\n").to_utf8();
                    response.push_all(&html);

                    resource.write(response.as_slice());

                    drop(resource);
                },
                _ => ()
            }

            loop {
                match window.poll() {
                    EventOption::Key(key_event) => {
                        if key_event.pressed{
                            if key_event.scancode == 1 {
                                break;
                            }
                        }
                    },
                    EventOption::None => break,
                    _ => ()
                }
            }
        }

        println!("Closed HTTP Server".to_string());

        loop {
            match window.poll() {
                EventOption::Key(key_event) => {
                    if key_event.pressed{
                        if key_event.scancode == 1 {
                            break;
                        }
                    }
                },
                EventOption::None => sys_yield(),
                _ => ()
            }
        }
    }
}

impl Application {
    pub fn new() -> Application {
        return Application {
            output: String::new(),
            scroll: Point::new(0, 0),
            wrap: true
        };
    }

    fn generate_html(&self, path: &String) -> String{
        let mut html = String::new();

        if *path == "readme".to_string() {
            html = html + "<title>Readme - Redox</title>\n";
        }else{
            html = html + "<title>Home - Redox</title>\n";
        }
        html = html + "<link rel='icon' href='data:;base64,iVBORw0KGgo='>\n";
        html = html + "<link rel='stylesheet' href='https://maxcdn.bootstrapcdn.com/bootstrap/3.3.4/css/bootstrap.min.css'>\n";
        html = html + "<link rel='stylesheet' href='https://maxcdn.bootstrapcdn.com/bootstrap/3.3.4/css/bootstrap-theme.min.css'>\n";
        html = html + "<script src='https://maxcdn.bootstrapcdn.com/bootstrap/3.3.4/js/bootstrap.min.js'></script>\n";

        html = html + "<div class='container'>\n";
            html = html + "<nav class='navbar navbar-default'>\n";
            html = html + "  <div class='container-fluid'>\n";
            html = html + "    <div class='navbar-header'>\n";
            html = html + "      <button type='button' class='navbar-toggle collapsed' data-toggle='collapse' data-target='#navbar-collapse'></button>\n";
            html = html + "      <a class='navbar-brand' href='/'>Redox Web Interface</a>\n";
            html = html + "    </div>\n";
            html = html + "    <div class='collapse navbar-collapse' id='navbar-collapse'>\n";
            html = html + "      <ul class='nav navbar-nav navbar-right'>\n";

            if *path == "readme".to_string() {
                html = html + "        <li><a href='/'>Home</a></li>\n";
                html = html + "        <li class='active'><a href='/readme'>Readme</a></li>\n";
            }else{
                html = html + "        <li class='active'><a href='/'>Home</a></li>\n";
                html = html + "        <li><a href='/readme'>Readme</a></li>\n";
            }

            html = html + "      </ul>\n";
            html = html + "    </div>\n";
            html = html + "  </div>\n";
            html = html + "</nav>\n";

            if *path == "readme".to_string() {
                let mut resource = URL::from_string(&"file:///README.md".to_string()).open();

                let mut resource_data: Vec<u8> = Vec::new();
                resource.read_to_end(&mut resource_data);
                html = html + "<div class='panel panel-default'>\n".to_string();
                    if resource_data.len() > 0 {
                        html = html + "<div class='panel-heading'>\n";
                            html = html + "<h3 class='panel-title'><span class='glyphicon glyphicon-book'></span> README</h3>";
                        html = html + "</div>\n";

                        html = html + "<div class='panel-body'>\n";
                            let mut in_code = false;
                            for line in String::from_utf8(&resource_data).split("\n".to_string()){
                                if line.starts_with("# ".to_string()){
                                    html = html + "<h1>" + encode(&line.substr(2, line.len() - 2)) + "</h1>\n";
                                }else if line.starts_with("## ".to_string()){
                                    html = html + "<h2>" + encode(&line.substr(3, line.len() - 3)) + "</h2>\n";
                                }else if line.starts_with("### ".to_string()){
                                    html = html + "<h3>" + encode(&line.substr(4, line.len() - 4)) + "</h3>\n";
                                }else if line.starts_with("- ".to_string()){
                                    html = html + "<li>" + encode(&line.substr(2, line.len() - 2)) + "</li>\n";
                                }else if line.starts_with("```".to_string()){
                                    if in_code {
                                        html = html + "</pre>\n";
                                        in_code = false;
                                    }else{
                                        html = html + "<pre>\n";
                                        in_code = true;
                                    }
                                }else{
                                    html = html + encode(&line);
                                    if in_code {
                                        html = html + "\n";
                                    }else{
                                        html = html + "<br/>\n";
                                    }
                                }
                            }
                            if in_code {
                                html = html + "</pre>\n";
                            }
                        html = html + "</div>\n";
                    }else{
                        html = html + "<div class='panel-heading'>\n";
                            html = html + "<h3 class='panel-title'><span class='glyphicon glyphicon-exlamation-sign'></span> Failed to open README</h3>\n";
                        html = html + "</div>\n";
                    }
                html = html + "</div>\n";
            }else{
                html = html + "<table class='table table-bordered'>\n".to_string();
                    let mut resource = URL::from_string(&path).open();

                    let resource_type;
                    match resource.stat() {
                        ResourceType::File => resource_type = "File".to_string(),
                        ResourceType::Dir => resource_type = "Dir".to_string(),
                        ResourceType::Array => resource_type = "Array".to_string(),
                        _ => resource_type = "None".to_string()
                    }

                    html = html + "  <caption><h3>" + encode(path) + "</h3><h4>" + encode(&resource_type) + "</h4></caption>\n";

                    let mut resource_data: Vec<u8> = Vec::new();
                    resource.read_to_end(&mut resource_data);
                    for line in String::from_utf8(&resource_data).split("\n".to_string()) {
                        html = html + "<tr><td>" + encode(&line) + "</td></tr>\n";
                    }
                html = html + "</table>\n";
            }

        html = html + "</div>\n";

        return html;
    }

    fn draw_content(&mut self, window: &mut Window){
        let scroll = self.scroll;

        let mut col = -scroll.x;
        let cols = window.content.width as isize / 8;
        let mut row = -scroll.y;
        let rows = window.content.height as isize / 16;

        {
            let content = &window.content;
            content.set(Color::new(0, 0, 0));

            for c in self.output.chars(){
                if self.wrap && col >= cols {
                    col = -scroll.x;
                    row += 1;
                }

                if c == '\n' {
                    col = -scroll.x;
                    row += 1;
                }else if c == '\t' {
                    col += 8 - col % 8;
                }else{
                    if col >= 0 && col < cols && row >= 0 && row < rows{
                        content.char(Point::new(8 * col, 16 * row), c, Color::new(224, 224, 224));
                    }
                    col += 1;
                }
            }

            if col > -scroll.x {
                col = -scroll.x;
                row += 1;
            }

            if self.wrap && col >= cols {
                col = -scroll.x;
                row += 1;
            }

            content.flip();

            RedrawEvent {
                redraw: REDRAW_ALL
            }.to_event().trigger();
        }

        if row >= rows {
            self.scroll.y += row - rows + 1;

            self.draw_content(window);
        }
    }
}
