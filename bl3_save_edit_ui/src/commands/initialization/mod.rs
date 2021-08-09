use bl3_save_edit_core::resources::INVENTORY_PARTS;

pub async fn load_lazy_data() {
    tokio_rayon::spawn(|| {
        let _ = &*INVENTORY_PARTS;
    })
    .await;
}
