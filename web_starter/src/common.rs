use crate::serde::deserialize_number;
use serde::{Deserialize, Serialize};
use validator::Validate;

const DEFAULT_PAGE: u64 = 1;
const DEFAULT_PAGE_SIZE: u64 = 10;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize,Validate)]
pub struct PaginationParams {
    #[validate(range(min = 1, message = "page must be greater than or equal to 1"))]
    #[serde(default = "default_page", deserialize_with = "deserialize_number")]
    pub page: u64,
    #[validate(range(min = 1,max= 100000, message = "page size must be less than or equal to 10000"))]
    #[serde(default = "default_page_size", deserialize_with = "deserialize_number")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    DEFAULT_PAGE
}

fn default_page_size() -> u64 {
    DEFAULT_PAGE_SIZE
}

#[derive(Debug, Serialize)]
pub struct Page<T> {
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
    pub items: Vec<T>,
}

impl<T> Page<T> {
    pub fn new(page: u64, page_size: u64, total: u64, items: Vec<T>) -> Self {
        Self {
            page,
            page_size,
            total,
            items,
        }
    }
}

impl PaginationParams {
    pub fn to_page<T>(&self, total: u64, items: Vec<T>) -> Page<T> {
        Page::new(self.page, self.page_size, total, items)
    }
}
