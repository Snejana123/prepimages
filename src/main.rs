mod imageworker;

use imageworker::imageworker::process_image;
use walkdir::WalkDir;

use clap::{Parser, command};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    // Каталог для обработки
    #[arg(short, long, default_value_t = String::from("."))]
    dir: String,

    //Размер квадрата изображения
    #[arg(short, long, default_value_t = 200)]
    image_size: u32,

    // Модель обнаружения лица , путь к файлу
    #[arg(short, long, default_value_t = String::from("seeta_fd_frontal_v1.0.bin"))]
    model_path: String,
}

fn main() {
    let args = Args::parse();

    println!("Кататалог изображений {}", args.dir);
    println!(
        "Размер квадрата выходного 8-бит серого изображения {}",
        args.image_size
    );
    println!("Путь к файлу модели обнаружения лица {}", args.model_path);

    //Взято из примера использования
    for entry in WalkDir::new(args.dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        //let sec = entry.metadata()?.modified()?;

        /*if f_name.ends_with(".json") && sec.elapsed()?.as_secs() < 86400 {
            println!("{}", f_name);
        }*/

        if entry.file_type().is_file() {
            let f_name = entry.into_path().into_os_string().into_string().unwrap();

            let (output_directory, file_name) = f_name.rsplit_once('/').unwrap();

            //пропустить gray - уже обработан
            if !file_name.starts_with("gray") {
                let result = process_image(
                    &args.model_path,
                    &f_name,
                    &output_directory,
                    args.image_size,
                    args.image_size,
                );
                match result {
                    Ok(name) => println!("Обработан {}", name),
                    Err(e) => println!("Ошибка {}", e),
                }
            }
        }
    }
}
