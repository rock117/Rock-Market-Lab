use entity::sea_orm::{ColumnTrait, EntityTrait, IdenStatic};
use std::fmt::Debug;
use strum::IntoEnumIterator;

/// 生成用于 OnConflict 的 update_columns，排除指定的列
/// 
/// # Arguments
/// * `all_columns` - 实体的所有列
/// * `exclude_columns` - 需要排除的列（通常是主键或不需要更新的列）
/// 
/// # Example
/// ```rust
/// use entity::ths_member;
/// 
/// let update_cols = get_update_columns_excluding(
///     &[
///         ths_member::Column::TsCode,
///         ths_member::Column::ConCode, 
///         ths_member::Column::ConName,
///         ths_member::Column::Weight,
///         ths_member::Column::IsNew,
///         ths_member::Column::InDate,
///         ths_member::Column::OutDate,
///     ],
///     &[ths_member::Column::TsCode, ths_member::Column::ConCode] // 排除主键
/// );
/// ```
pub fn get_update_columns_excluding<T>(all_columns: &[T], exclude_columns: &[T]) -> Vec<T>
where
    T: Clone + PartialEq,
{
    all_columns
        .iter()
        .filter(|col| !exclude_columns.contains(col))
        .cloned()
        .collect()
}

/// 动态获取实体的所有列，排除指定列
/// 
/// # Arguments
/// * `exclude_columns` - 需要排除的列（通常是主键或不需要更新的列）
/// 
/// # Example
/// ```rust
/// use entity::ths_member;
/// 
/// let update_cols = get_entity_update_columns::<ths_member::Entity>(&[
///     ths_member::Column::TsCode,
///     ths_member::Column::ConCode,
/// ]);
/// ```
pub fn get_entity_update_columns<E>(
    exclude_columns: &[<E as EntityTrait>::Column]
) -> Vec<<E as EntityTrait>::Column>
where
    E: EntityTrait,
    <E as EntityTrait>::Column: ColumnTrait + Clone + PartialEq + Debug,
{
    // 获取实体的所有列
    let all_columns = <E as EntityTrait>::Column::iter().collect::<Vec<_>>();
    
    get_update_columns_excluding(&all_columns, exclude_columns)
}

/// 针对 ths_member 表的便捷函数（使用动态列获取）
pub fn get_ths_member_update_columns(exclude_columns: &[entity::ths_member::Column]) -> Vec<entity::ths_member::Column> {
    get_entity_update_columns::<entity::ths_member::Entity>(exclude_columns)
}
