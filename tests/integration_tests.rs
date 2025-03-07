extern crate ceph;
extern crate serde;
extern crate serde_json;

use ceph::cmd::{ClusterHealth, CrushTree, MonStatus, OsdMetadata, OsdInfo};
use std::fs::File;
use std::io::Read;

#[test]
fn test_ceph_health_jewel() {
    let json = {
        let mut buff = String::new();
        let mut f = File::open("tests/ceph_health-jewel").unwrap();
        f.read_to_string(&mut buff).unwrap();
        buff
    };
    let status: ClusterHealth = serde_json::from_str(&json).unwrap();
    println!("cluster_health: {:#?}", status);
}

#[test]
fn test_mon_status_hammer() {
    let json = {
        let mut buff = String::new();
        let mut f = File::open("tests/mon_status-hammer").unwrap();
        f.read_to_string(&mut buff).unwrap();
        buff
    };
    let status: MonStatus = serde_json::from_str(&json).unwrap();
    println!("mon_status: {:#?}", status);
}

#[test]
fn test_mon_status_jewel() {
    let json = {
        let mut buff = String::new();
        let mut f = File::open("tests/mon_status-jewel").unwrap();
        f.read_to_string(&mut buff).unwrap();
        buff
    };
    let status: MonStatus = serde_json::from_str(&json).unwrap();
    println!("mon_status: {:#?}", status);
}

#[test]
fn test_mon_status_nautilus() {
    let json = {
        let mut buff = String::new();
        let mut f = File::open("tests/mon_status-nautilus").unwrap();
        f.read_to_string(&mut buff).unwrap();
        buff
    };
    let status: MonStatus = serde_json::from_str(&json).unwrap();
    println!("mon_status: {:#?}", status);
}

#[test]
fn test_osd_tree_hammer() {
    let json = {
        let mut buff = String::new();
        let mut f = File::open("tests/osd_tree-hammer").unwrap();
        f.read_to_string(&mut buff).unwrap();
        buff
    };
    let tree: CrushTree = serde_json::from_str(&json).unwrap();
    println!("osd_tree: {:#?}", tree);
}

#[test]
fn test_osd_tree_jewel() {
    let json = {
        let mut buff = String::new();
        let mut f = File::open("tests/osd_tree-jewel").unwrap();
        f.read_to_string(&mut buff).unwrap();
        buff
    };
    let tree: CrushTree = serde_json::from_str(&json).unwrap();
    println!("osd_tree: {:#?}", tree);
}

#[test]
fn test_osd_tree_pacific() {
    let json = {
        let mut buff = String::new();
        let mut f = File::open("tests/osd_tree_pacific").unwrap();
        f.read_to_string(&mut buff).unwrap();
        buff
    };
    let tree: CrushTree = serde_json::from_str(&json).unwrap();
    println!("osd_tree: {:#?}", tree);
}


#[test]
fn test_osd_metadata_pacific() {
    let json = {
        let mut buff = String::new();
        let mut f = File::open("tests/osd_metadata_pacific").unwrap();
        f.read_to_string(&mut buff).unwrap();
        buff
    };
    let md: Vec<OsdMetadata> = serde_json::from_str(&json).unwrap();
    println!("osd_metadata: {:#?}", md);
}

#[test]
fn test_osd_info_pacific() {
    let json = {
        let mut buff = String::new();
        let mut f = File::open("tests/osd_info_pacific").unwrap();
        f.read_to_string(&mut buff).unwrap();
        buff
    };
    let md: Vec<OsdInfo> = serde_json::from_str(&json).unwrap();
    println!("osd_info: {:#?}", md);
}
