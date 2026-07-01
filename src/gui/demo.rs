//! Synthetic engineering catalog for dashboard preview without admin elevation.

use crate::catalog::{Catalog, FileTimestamps};

pub fn build_demo_catalog() -> Catalog {
    let mut cat = Catalog::new('C');

    let files: &[(&str, &str, u64, &str)] = &[
        (r"\Projects\Drawings\ABC-100_REV-A.pdf", "ABC-100_REV-A.pdf", 245_760, "Drawing"),
        (r"\Projects\Drawings\ABC-100_REV-A.dwg", "ABC-100_REV-A.dwg", 512_000, "Drawing"),
        (r"\Projects\Drawings\ABC-100_REV-B.pdf", "ABC-100_REV-B.pdf", 248_000, "Drawing"),
        (r"\Projects\Drawings\XYZ-200.pdf", "XYZ-200.pdf", 180_000, "Drawing"),
        (r"\CAD\Models\ABC-100_REV-A.sldprt", "ABC-100_REV-A.sldprt", 2_400_000, "CAD Model"),
        (r"\CAD\Models\ABC-100_REV-A.step", "ABC-100_REV-A.step", 1_800_000, "CAD Model"),
        (r"\CAD\Models\XYZ-200.stp", "XYZ-200.stp", 900_000, "CAD Model"),
        (r"\Quality\FMEA\Widget-FMEA-2024.xlsx", "Widget-FMEA-2024.xlsx", 89_000, "Quality"),
        (r"\Quality\RCA\Line3-RCA-8D.xlsx", "Line3-RCA-8D.xlsx", 120_000, "Quality"),
        (r"\Quality\NCR\NCR-1042.pdf", "NCR-1042.pdf", 45_000, "Quality"),
        (r"\Test\FEA\Bracket-SIM-report.csv", "Bracket-SIM-report.csv", 34_000, "Test/Simulation Report"),
        (r"\Suppliers\Quotes\QTN-8842.pdf", "QTN-8842.pdf", 210_000, "Supplier/Quote"),
        (r"\Mfg\Production\BOM-Widget-v3.xlsx", "BOM-Widget-v3.xlsx", 55_000, "Manufacturing/Production Data"),
        (r"\Correspondence\PO-confirm-8842.pdf", "PO-confirm-8842.pdf", 32_000, "Correspondence"),
        (r"\Temp\misc-data.bin", "misc-data.bin", 4096, "Other"),
        (r"\Temp\unknown-file.dat", "unknown-file.dat", 8192, "Unclassified"),
    ];

    cat.upsert_entry(5, 1, 5, 1, "", "\\", 0, FileTimestamps::default(), true, true);

    for (idx, (path, name, size, _cat_hint)) in files.iter().enumerate() {
        let rec = 100 + idx as u64;
        let parent = 5u64;
        cat.upsert_entry(
            rec,
            1,
            parent,
            1,
            name,
            path,
            *size,
            FileTimestamps {
                modified: 1_700_000_000 + idx as u64,
                ..FileTimestamps::default()
            },
            true,
            true,
        );
    }

    cat
}
