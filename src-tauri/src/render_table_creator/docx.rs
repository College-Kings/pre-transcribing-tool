use crate::error::Result;
use crate::render_table_creator::scene_item::SceneItem;
use docx_rs::{BreakType, Docx, Paragraph, Run, RunFonts, Table, TableCell, TableRow, WidthType};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use tauri::api::dialog::blocking::FileDialogBuilder;

fn create_table(data: &Vec<SceneItem>) -> Table {
    let mut rows = Vec::with_capacity(data.len() + 1);

    rows.push(TableRow::new(vec![
        TableCell::new().width(1250, WidthType::Auto).add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .fonts(RunFonts::new().ascii("Arial"))
                    .size(24)
                    .add_text("Scene ID"),
            ),
        ),
        TableCell::new().width(7250, WidthType::Auto).add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .fonts(RunFonts::new().ascii("Arial"))
                    .size(24)
                    .add_text("Description"),
            ),
        ),
        TableCell::new().width(500, WidthType::Auto).add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .fonts(RunFonts::new().ascii("Arial"))
                    .size(24)
                    .add_text("Occurrences"),
            ),
        ),
    ]));

    for d in data {
        rows.push(TableRow::new(vec![
            TableCell::new().width(1250, WidthType::Auto).add_paragraph(
                Paragraph::new().add_run(
                    Run::new()
                        .fonts(RunFonts::new().ascii("Arial"))
                        .size(24)
                        .add_text(&d.id),
                ),
            ),
            TableCell::new().width(7250, WidthType::Auto).add_paragraph(
                Paragraph::new().add_run(
                    Run::new()
                        .fonts(RunFonts::new().ascii("Arial"))
                        .size(24)
                        .add_text(&d.description),
                ),
            ),
            TableCell::new().width(500, WidthType::Auto).add_paragraph(
                Paragraph::new().add_run(
                    Run::new()
                        .fonts(RunFonts::new().ascii("Arial"))
                        .size(24)
                        .add_text(d.occurrences.to_string()),
                ),
            ),
        ]))
    }

    Table::new(rows)
}

pub fn create_doc(
    scene_number: &str,
    notes: HashMap<String, Vec<String>>,
    notes_order: Vec<String>,
    scene_items: Vec<SceneItem>,
    total_render_count: i32,
) -> Result<()> {
    let table = create_table(&scene_items);

    let mut scene_notes = Run::new().fonts(RunFonts::new().ascii("Arial")).size(24);
    for key in notes_order {
        for value in notes.get(&key).unwrap() {
            scene_notes = scene_notes.add_text(&format!("{}: {}", key, value));
            scene_notes = scene_notes.add_break(BreakType::TextWrapping);
        }
    }

    let scene_items_len = scene_items.len() as f32;

    let mut render_notes = Run::new().fonts(RunFonts::new().ascii("Arial")).size(24);
    render_notes = render_notes.add_text(format!("Total Renders: {}", total_render_count));
    render_notes = render_notes.add_break(BreakType::TextWrapping);
    render_notes = render_notes.add_text(format!("Unique Renders: {}", scene_items_len));
    render_notes = render_notes.add_break(BreakType::TextWrapping);
    render_notes = render_notes.add_text(format!(
        "Reused: {}%",
        (100.0 - (scene_items_len / total_render_count as f32 * 100.0)).round()
    ));

    let docx = Docx::new()
        .add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .add_text(format!("Scene {} Render Table", scene_number))
                    .fonts(RunFonts::new().ascii("Arial"))
                    .size(56),
            ),
        )
        .add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .add_text("Scene Notes")
                    .fonts(RunFonts::new().ascii("Arial"))
                    .size(32),
            ),
        )
        .add_paragraph(Paragraph::new().add_run(scene_notes))
        .add_paragraph(Paragraph::new().add_run(render_notes))
        .add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .add_text("Renders")
                    .fonts(RunFonts::new().ascii("Arial"))
                    .size(36),
            ),
        )
        .add_table(table);

    save_docx_file(docx, scene_number)?;

    Ok(())
}

fn save_docx_file(docx: Docx, scene_number: &str) -> Result<()> {
    if let Some(path) = FileDialogBuilder::new()
        .set_file_name(&format!("scene{}", scene_number))
        .set_title("Save DOCX File")
        .add_filter("Word Document (*.docx)", &["docx"])
        .save_file()
    {
        let file = File::create(path)?;
        docx.build().pack(file).map_err(io::Error::from)?;
    }

    Ok(())
}
