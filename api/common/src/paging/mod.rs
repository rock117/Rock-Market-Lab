
/// 分页
///
/// # Arguments
///
/// * `datas` - 数据
/// * `page` - 页码
/// * `page_size` - 每页的记录数
pub fn get_paging_data<T: Clone>(datas: &[T], page: usize, page_size: usize) -> Vec<T> {
    let start = (page - 1) * page_size;
    let end = (start + page_size).min(datas.len());
    datas[start..end].to_vec()
}