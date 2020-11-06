fn main() {
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-search=/home/ros/workspace/zenoh_dds_dep_ws/install/lib");
    println!("cargo:rustc-link-search=/home/evshary/workspace/zenoh_dds_dep_ws/install/lib");
    println!("cargo:rustc-link-lib=cdds-util");
}
