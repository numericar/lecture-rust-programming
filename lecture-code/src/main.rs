// mod lec_003_variable;
// mod lec_004_datatype;
// mod lec_006_function;
// mod lec_007_controlflow;
// mod lec_008_loop;
// mod lec_009_ownership;
// mod lec_010_borrowing;
// mod lec_011_reference;
// mod lec_012_struct;
// mod lec_013_enums;
// mod lec_014_err_handling;
// mod lec_016_option;
// mod lec_017_collection;
// mod lec_018_string;
// mod lec_019_hash_map;
// mod lec_020_generic;
// mod lec_021_trait;
// mod lec_022_def_over;
// mod lec_023_associated;
// mod lec_024_dynamic_dispatch;
// mod lec_026_smart_pointer;
// mod lec_027_lifetime;
// mod lec_028_threading;
// mod lec_029_channels;
mod lec_030_tokio;

use tokio;

#[tokio::main]
async fn main() {
    lec_030_tokio::process().await;
}