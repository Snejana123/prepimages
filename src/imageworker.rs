pub mod imageworker {

    use std::error::Error;

    use image::imageops::FilterType;
    use image::{DynamicImage, GenericImage, GrayImage, Rgb};
    use imageproc::drawing::draw_hollow_rect_mut;
    use imageproc::rect::Rect;
    use rustface::{Detector, FaceInfo, ImageData};

    use rustface;

    /// face_model_file_path: &str,  модель обнаружения лиц
    /// source_image_file: &str, исходный файл
    /// output_directory: &str, каталог для сохранения вырезанного изображения
    /// new_image_width: u32, ширина вырезанного изображени
    /// new_image_height: u32, высота вырезанного изображения
    /// размер вырезанного изображения должен быть квадратом,

    pub fn process_image(
        face_model_file_path: &str,
        source_image_file: &str,
        output_directory: &str,
        new_image_width: u32,
        new_image_height: u32,
    ) -> Result<String, Box<dyn Error>> {
        run_model(
            face_model_file_path,
            source_image_file,
            output_directory,
            new_image_width,
            new_image_height,
        )
    }

    fn run_model(
        model_path: &str,
        file_path: &str,
        ouput_directory: &str,
        n_width: u32,
        n_height: u32,
    ) -> Result<String, Box<dyn Error>> {
        let mut detector = rustface::create_detector(model_path).unwrap();
        /* Параметры из базового примера использования модели
         detector.set_min_face_size(20);
         detector.set_score_thresh(2.0);
         detector.set_pyramid_scale_factor(0.8);
         detector.set_slide_window_step(4, 4);
        */
        detector.set_min_face_size(20);
        detector.set_score_thresh(2.0);
        detector.set_pyramid_scale_factor(0.8);
        detector.set_slide_window_step(4, 4);

        let source_file_name = file_path.split("/").last().unwrap();
        println!("Processing {} ...", file_path);

        let image: DynamicImage = image::open(file_path)?;
        let mut rgb = image.to_rgb8();

        let gray = image.to_luma8();
        let faces = detect_faces(&mut *detector, &gray);

        let mut face_num = 0;

        for face in faces {
            face_num += 1;
            let bbox = face.bbox();
            let rect = Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height());
            //draw_hollow_rect_mut(&mut rgb, rect, Rgb([255, 0, 0]));
            let mut gr = image.to_luma8();
            //проблема с размером отрицательная координата
            println!("image {}, {}", gr.width(), gr.height());
            println!(
                "bbox {}, {}, {}, {}",
                bbox.x(),
                bbox.y(),
                bbox.width(),
                bbox.height()
            );
            let check_whidth = gr.width() as i64 - (bbox.x() as i64 + bbox.width() as i64);
            let check_height = gr.height() as i64 - (bbox.y() as i64 + bbox.height() as i64);
            //пропускаем если модель нашла границы лица за пределами исходного изображения
            if bbox.x() < 0 || bbox.y() < 0 || check_whidth < 0 || check_height < 0 {
                continue;
            }
            let gr = gr
                .sub_image(
                    bbox.x() as u32,
                    bbox.y() as u32,
                    bbox.width() as u32,
                    bbox.height() as u32,
                )
                .to_image();
            let gray_save_path = format!(
                "{}/gray_{}_{}.png",
                ouput_directory, face_num, source_file_name
            );
            let _ = gr.save(gray_save_path.clone());

            let mut gray_d_image: DynamicImage = image::open(gray_save_path.clone())?;
            gray_d_image = gray_d_image.resize_exact(n_width, n_height, FilterType::Lanczos3);
            let gray_scaled_save_path = format!(
                "{}/gray_scaled_{}_{}.png",
                ouput_directory, face_num, source_file_name
            );
            let _ = gray_d_image.save(gray_scaled_save_path);

            //rgb
            let prgb = rgb
                .sub_image(
                    bbox.x() as u32,
                    bbox.y() as u32,
                    bbox.width() as u32,
                    bbox.height() as u32,
                )
                .to_image();
            let rgb_save_path = format!(
                "{}/rgb_{}_{}.png",
                ouput_directory, face_num, source_file_name
            );
            let _ = prgb.save(rgb_save_path.clone());

            let mut rgb_d_image: DynamicImage = image::open(rgb_save_path.clone())?;
            rgb_d_image = rgb_d_image.resize_exact(n_width, n_height, FilterType::Lanczos3);
            let rgb_scaled_save_path = format!(
                "{}/rgb_scaled_{}_{}.png",
                ouput_directory, face_num, source_file_name
            );
            let _ = rgb_d_image.save(rgb_scaled_save_path);
        }

        Ok(file_path.into())
    }

    fn detect_faces(detector: &mut dyn Detector, gray: &GrayImage) -> Vec<FaceInfo> {
        let (width, height) = gray.dimensions();
        let mut image = ImageData::new(gray, width, height);
        let faces = detector.detect(&mut image);
        faces
    }
}
