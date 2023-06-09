use std::{fs::File, process::Command};
use std::io::Write;
use tera::{Context, Tera};

fn main() {
    let template_path = "src/template.tex";
    let template_content = std::fs::read_to_string(template_path).expect("Failed to read the template file.");

    let mut tera = match Tera::new("templates/**/*.tex") {
        Ok(t) => t,
        Err(e) => {
            println!("Error parsing templates: {}", e);
            return;
        }
    };

    let mut context = Context::new();
    context.insert("name", "Dharmik Jethva");
    context.insert("age", &19);

    let rendered = match tera.render_str(&template_content, &context) {
        Ok(r) => r,
        Err(e) => {
            println!("Error rendering template: {}", e);
            return;
        }
    };

    let output_path = "output.tex";
    let mut output_file = File::create(output_path).expect("Failed to create the output file.");
    output_file
        .write_all(rendered.as_bytes())
        .expect("Failed to write to the output file.");

    println!("Latex file generated successfully.");

    //pdflatex ke liye texlive-latex-base install karna padega
    let output_pdf = "output.pdf";
    let latex_command = format!("pdflatex -output-directory=. {}", output_path);
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(latex_command)
        .output()
        .expect("Failed to execute LaTeX command.");

    if output.status.success() {
        std::fs::rename("output.pdf", output_pdf).expect("Failed to rename PDF file.");
        println!("PDF generated successfully.");

                // Convert PDF to image using pdftoppm
                let output_image = "output.jpg";
                let pdftoppm_command = format!("pdftoppm -jpeg {} {}", output_pdf, output_image);
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(pdftoppm_command)
                    .output()
                    .expect("Failed to execute pdftoppm command.");
        
                if output.status.success() {
                    println!("Image generated successfully.");
                } else {
                    let error_message = String::from_utf8_lossy(&output.stderr);
                    println!("Error generating image: {}", error_message);
                }
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!("Error generating PDF: {}", error_message);
    }
}

