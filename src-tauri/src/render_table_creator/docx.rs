use crate::render_table_creator::config::HEADER_KEYS;
use crate::render_table_creator::scene_item::SceneItem;
use docx_rs::{BreakType, Docx, Paragraph, Run, Table, TableCell, TableRow, WidthType};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

fn create_table(data: &Vec<SceneItem>) -> Table {
    let mut rows = Vec::with_capacity(data.len() + 1);

    rows.push(TableRow::new(vec![
        TableCell::new()
            .width(1250, WidthType::Auto)
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Scene ID"))),
        TableCell::new()
            .width(7250, WidthType::Auto)
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Description"))),
        TableCell::new()
            .width(500, WidthType::Auto)
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Occurrences"))),
    ]));

    for d in data {
        rows.push(TableRow::new(vec![
            TableCell::new()
                .width(1250, WidthType::Auto)
                .add_paragraph(Paragraph::new().add_run(Run::new().add_text(&d.id))),
            TableCell::new()
                .width(7250, WidthType::Auto)
                .add_paragraph(Paragraph::new().add_run(Run::new().add_text(&d.description))),
            TableCell::new().width(500, WidthType::Auto).add_paragraph(
                Paragraph::new().add_run(Run::new().add_text(d.occurrences.to_string())),
            ),
        ]))
    }

    Table::new(rows)
}

pub fn create_doc(
    notes: HashMap<String, String>,
    scene_items: Vec<SceneItem>,
    total_render_count: i32,
) {
    let table = create_table(&scene_items);

    let path = Path::new(r"D:\Crimson Sky\College Kings\College-Kings-2\game\ep4\scene1a.docx");
    let file = File::create(path).unwrap();

    let mut scene_notes = Run::new();
    let empty_string = String::new();
    for key in HEADER_KEYS {
        let x = notes.get(key).unwrap_or(&empty_string);
        scene_notes = scene_notes.add_text(&format!("{}: {}", key, x));
        scene_notes = scene_notes.add_break(BreakType::TextWrapping);
    }

    let scene_items_len = scene_items.len() as f32;

    let mut render_notes = Run::new();
    render_notes = render_notes.add_text(&format!("Total Renders: {}", total_render_count));
    render_notes = render_notes.add_break(BreakType::TextWrapping);
    render_notes = render_notes.add_text(&format!("Unique Renders: {}", scene_items_len));
    render_notes = render_notes.add_break(BreakType::TextWrapping);
    render_notes = render_notes.add_text(&format!(
        "Reused: {}%",
        (100.0 - (scene_items_len / total_render_count as f32 * 100.0)).round()
    ));

    Docx::new()
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text("Scene 1a Render Table").size(56)),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Scene Notes").size(32)))
        .add_paragraph(Paragraph::new().add_run(scene_notes))
        .add_paragraph(Paragraph::new().add_run(render_notes))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Renders").size(36)))
        .add_table(table)
        .build()
        .pack(file)
        .unwrap();
}
