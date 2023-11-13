use crate::render_table_creator::main::SceneItem;
use docx_rs::{Docx, Paragraph, Run, Style, StyleType, Table, TableCell, TableRow, WidthType};
use std::fs::File;
use std::path::Path;

fn create_table(data: Vec<SceneItem>) -> Table {
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
                .add_paragraph(Paragraph::new().add_run(Run::new().add_text(d.id))),
            TableCell::new()
                .width(7250, WidthType::Auto)
                .add_paragraph(Paragraph::new().add_run(Run::new().add_text(d.description))),
            TableCell::new().width(500, WidthType::Auto).add_paragraph(
                Paragraph::new().add_run(Run::new().add_text(d.occurrences.to_string())),
            ),
        ]))
    }

    Table::new(rows)
}

pub fn create_doc(scene_items: Vec<SceneItem>) {
    let table = create_table(scene_items);

    let path = Path::new(r"D:\Crimson Sky\College Kings\College-Kings-2\game\ep4\scene1a.docx");
    let file = File::create(path).unwrap();

    Docx::new()
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text("Scene 1a Render Table").size(56)),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Scene Notes").size(32)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello World!")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Renders").size(36)))
        .add_table(table)
        .build()
        .pack(file)
        .unwrap();
}
