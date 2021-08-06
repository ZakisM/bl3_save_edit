use bl3_save_edit_core::resources::{INVENTORY_PARTS_ALL, INVENTORY_SERIAL_DB};

pub async fn load_lazy_data() {
    tokio_rayon::spawn(|| {
        let _ = &*INVENTORY_SERIAL_DB;
        let _ = &*INVENTORY_PARTS_ALL;
    })
    .await;
}
