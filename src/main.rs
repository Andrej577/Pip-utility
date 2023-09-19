#![windows_subsystem = "windows"]

use dioxus::prelude::*;

use std::process::Command;

use std::path::Path;

fn main() {
    dioxus_desktop::launch(app)
}

fn app(cx: Scope) -> Element
{
    let location = use_state(cx, || "".to_string());
    let name = use_state(cx, || "".to_string());
    let package_name = use_state(cx, || "".to_string());
    let path_str = format!("{}{}", &location, r#"\Scripts"#);
    let path = Path::new(&path_str);
    let greska = use_state(cx, || "".to_string());

    let mut button_state_create = false;
    let mut button_state_activate = false;


    cx.render(rsx!(
        unsafe{
            let tl = tasklist::Tasklist::new();
            for i in tl{
                if i.get_pname() == "WindowsTerminal.exe"
                {
                    button_state_activate = true;
                }
            }
        }
        head 
        {
            link 
            {
                link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/bootstrap@5.1.1/dist/css/bootstrap.min.css" }
                link { rel: "script", href: "https://cdn.jsdelivr.net/npm/bootstrap@5.1.1/dist/js/bootstrap.bundle.min.js" }
            }
        }
        div
        {   
            class: "col",
            div
            {
                class: "row",
                input
                {   
                    placeholder: "Unesi putanju",
                    class: "col form-control m-4",
                    value: "{location}",
                    oninput: move |e|{location.set(e.value.clone())}
                }
                input
                {   
                    class: "col m-4",
                    r#type: "file",
                    directory: true,
                    onchange: |evt| 
                    {
                        to_owned![location];
                        async move 
                        {
                            if let Some(file_engine) = &evt.files 
                            {
                                let files = file_engine.files();
                                for file_name in files 
                                {
                                    //sleep(std::time::Duration::from_secs(1)).await;
                                    location.set(file_name.clone());
                                }
                            }
                        }
                    }
                }
            }
            if path.exists()
            {
                button_state_create = true;
                Some(rsx!
                (
                    p
                    {
                        class: "ml-4",
                        style: "margin-left: 20px;",
                        "Virtual env za ovaj folder vec postoji",
                    }
                ))
            }
            div
            {
                class: "row",
                input
                {   
                    placeholder: "Unesi ime projekta",
                    class: "col form-control m-3",
                    value: "{name}",
                    oninput: move |e|
                    {
                        name.set(e.value.clone())
                    },
                }
                button
                {  
                    disabled: button_state_create, 
                    class: "col btn btn-outline-primary m-3",
                    onclick: move |_| 
                    {
                        location.set(pip_create(location.clone().to_string(), name.clone().to_string()));

                    },
                    "Create"
                }
            }
            div
            {
                button
                {   
                    disabled: button_state_activate,
                    class: "col btn btn-outline-primary m-3",
                    onclick: move |_| 
                    {
                        pokreni_cmd(location.clone().to_string());
                        //button_state.set(true)
                    }
                    ,
                    "Acitvate"
                }
            }
            div
            {
                class: "row",
                input
                {   
                    placeholder: "Unesi ime paketa",
                    class: "col form-control m-3",
                    value: "{package_name}",
                    oninput: move |e|
                    {
                        package_name.set(e.value.clone())
                    }
                }
                button
                {   
                    class: "col btn btn-outline-primary m-3",
                    onclick: move |_| 
                    {
                        install_package(location.to_string(), package_name.to_string())
                    },
                    "Instaliraj paket"
                }
            }
            button
            {
                class: "col btn btn-outline-primary m-3",
                onclick: move |_| 
                {
                    greska.set(izlistaj_pakete(location.to_string()));
                },
                "Prikazi pakete"
            }
            div
            {

                p
                {
                    style: "max-width: 150px;", 
                    class: "ml-4 text-wrap",
                    "{greska}"
                }
            }
        }
    ))
}

fn pokreni_cmd(location: String)
{
    let mut cmd = Command::new("cmd");
    //let path = format!(r#"C:\Users\andre\OneDrive\Radna površina\PIP PROBA\Proba\Scripts\"#);
    let path = format!("{}{}", &location, r#"\Scripts\"#);

    cmd.arg("/K");
    cmd.arg(format!("cd {} && activate", &path));
    //cmd.arg("activate");

    match cmd.spawn() 
    {
        Ok(_) => println!("Cmd pokrenut"),
        Err(e) => println!("{} {}", &path, e)
    }
}

fn pip_create(location: String, name: String) -> String 
{   
    let mut cmd = Command::new("cmd");
    let path = format!("{}\\{}", &location, &name);


    cmd.args(&["/C","py", "-m", "venv", &name])
        .current_dir(&location);
                
    match cmd.spawn() 
    {
        Ok(_) => println!("Naredba ok"),
        Err(e) => println!("{}", e)
    }   

    path
}

fn install_package(location: String, package_name: String) 
{
    let path = format!("{}{}", &location, r#"\Scripts\"#);
    let mut cmd = Command::new("cmd");

    
    cmd.args(&["/C","python", "-m", "pip", "install", &package_name])
        .current_dir(&path);

    match cmd.spawn() 
    {
        Ok(_) => println!("Naredba ok"),
        Err(e) => println!("{}", e)
    } 
    
}

fn izlistaj_pakete(location: String) -> String 
{
    let path = format!("{}{}", location, r#"\Scripts\"#);

    let output = Command::new("cmd")
        .args(&["/C", "python", "-m", "pip", "list"])
        .current_dir(&path)
        .output()
        .expect("Greška prilikom izvršavanja komande");

    if output.status.success() {
        // Konvertujte izlaz u string
        let output_string = String::from_utf8_lossy(&output.stdout).to_string();
        output_string
    } else {
        panic!("Izvršavanje komande nije uspelo: {:?}", output.status);
    }
}

    
