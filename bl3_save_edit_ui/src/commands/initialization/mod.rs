use bl3_save_edit_core::resources::{
    INVENTORY_PARTS_ALL_CATEGORIZED, INVENTORY_SERIAL_DB_PARTS_CATEGORIZED,
};

pub async fn load_lazy_data() {
    tokio_rayon::spawn(|| {
        let _ = &*INVENTORY_PARTS_ALL_CATEGORIZED;
        let _ = &*INVENTORY_SERIAL_DB_PARTS_CATEGORIZED;
    })
    .await;
}
